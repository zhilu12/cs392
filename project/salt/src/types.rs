use crate::utils::{Expr, Ident, Lifetime, Lval, Mutable, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Int,
    Box(Box<Type>),
    Ref(Lval, Mutable),
    Undefined(Box<Type>),
}

impl Type {
    pub fn boxx(t: Type) -> Self {
        Type::Box(Box::new(t))
    }

    pub fn undefined(t: Type) -> Self {
        Type::Undefined(Box::new(t))
    }

    pub fn imm_ref(lval: Lval) -> Self {
        Type::Ref(lval, false)
    }

    pub fn mut_ref(lval: Lval) -> Self {
        Type::Ref(lval, true)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub tipe: Type,
    pub lifetime: Lifetime,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Env(pub HashMap<Ident, Slot>);

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownVar(String),
    CannotDeref(Type),
    MovedOut(Lval),
    MoveBehindRef(Lval),
    UpdateBehindImmRef(Lval),
    CopyAfterMutBorrow(Lval),
    MoveAfterBorrow(Lval),
    MutBorrowBehindImmRef(Lval),
    MutBorrowAfterBorrow(Lval),
    BorrowAfterMutBorrow(Lval),
    Shadowing(String),
    IncompatibleTypes(Type, Type),
    LifetimeTooShort(Expr),
    AssignAfterBorrow(Lval),
}

pub type TypeResult<T> = Result<T, Error>;

impl Env {
    pub fn insert(&mut self, var: &str, tipe: Type, lifetime: Lifetime) {
        self.0.insert(var.to_string(), Slot { tipe, lifetime });
    }

    pub fn type_lval(&self, lval: &Lval) -> TypeResult<Slot> {
        let (ident, derefs) = (&lval.ident, lval.derefs);
        let mut slot = self
            .0
            .get(ident)
            .ok_or_else(|| Error::UnknownVar(ident.clone()))?
            .clone();

        for _ in 0..derefs {
            match &slot.tipe {
                Type::Box(inner) => slot.tipe = *inner.clone(),
                Type::Ref(ref lval, _) => {
                    slot = self.type_lval(lval)?;
                }
                _ => return Err(Error::CannotDeref(slot.tipe.clone())),
            }
        }
        Ok(slot)
    }

    // Returns the type under the boxes of a type, given that the
    // underlying type is defined
    pub fn contained(&self, var: &str) -> Option<&Type> {
        self.0.get(var).and_then(|slot| {
            let mut t = &slot.tipe;
            while let Type::Box(inner) = t {
                t = inner.as_ref();
            }
            match t {
                Type::Undefined(_) => None,
                _ => Some(t),
            }
        })
    }

    pub fn read_prohibited(&self, lval: &Lval) -> bool {
        self.type_lval(lval)
            .map(|slot| matches!(slot.tipe, Type::Undefined(_)))
            .unwrap_or(true)
    }

    pub fn write_prohibited(&self, lval: &Lval) -> bool {
        self.type_lval(lval)
            .map(|slot| matches!(slot.tipe, Type::Ref(_, false) | Type::Undefined(_)))
            .unwrap_or(true)
    }

    // "move" is a keyword in Rust
    pub fn moove(&mut self, lval: &Lval) -> TypeResult<()> {
        let (ident, derefs) = (&lval.ident, lval.derefs);
        let slot = self
            .0
            .get_mut(ident)
            .ok_or_else(|| Error::UnknownVar(ident.clone()))?;

        let mut t = &mut slot.tipe;
        for _ in 0..derefs {
            match t {
                Type::Box(inner) => {
                    t = inner.as_mut();
                }
                _ => return Err(Error::MoveBehindRef(lval.clone())),
            }
        }

        if matches!(t, Type::Ref(_, _)) {
            return Err(Error::MoveBehindRef(lval.clone()));
        }

        *t = Type::Undefined(Box::new(std::mem::replace(t, Type::Unit)));
        Ok(())
    }

    // so is "mut"
    pub fn muut(&self, lval: &Lval) -> bool {
        let slot = match self.0.get(&lval.ident) {
            Some(s) => s,
            None => return false,
        };
        let mut t = slot.tipe.clone();
        let mut rem = lval.derefs;

        loop {
            if rem > 0 {
                match t {
                    Type::Box(inner) => {
                        // consume one `*` over a Box<T>
                        t = *inner;
                        rem -= 1;
                    }
                    Type::Ref(inner_lval, is_mut) => {
                        // consume one `*` over a &T or &mut T
                        if !is_mut {
                            return false;
                        }
                        // follow the referent
                        if let Some(next_slot) = self.0.get(&inner_lval.ident) {
                            t = next_slot.tipe.clone();
                            rem = inner_lval.derefs;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            } else {
                // no more explicit `*` in the Lval — but if we still have a &T, we must check it too
                match t {
                    Type::Box(inner) => {
                        // keep drilling down past any leftover boxes
                        t = *inner;
                    }
                    Type::Ref(inner_lval, is_mut) => {
                        if !is_mut {
                            return false;
                        }
                        if let Some(next_slot) = self.0.get(&inner_lval.ident) {
                            t = next_slot.tipe.clone();
                            rem = inner_lval.derefs;
                        } else {
                            return false;
                        }
                    }
                    _ => return true, // reached a non-reference, non-box type
                }
            }
        }
    }

    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool {
        use Type::*;

        match (t1, t2) {
            (Undefined(inner1), _) => self.compatible(inner1, t2),
            (_, Undefined(inner2)) => self.compatible(t1, inner2),
            (Int, Int) | (Unit, Unit) => true,
            (Box(a), Box(b)) => self.compatible(a, b),
            (Ref(_, m1), Ref(_, m2)) => m1 == m2,
            _ => false,
        }
    }

    pub fn write(&mut self, lval: &Lval, tipe: Type) -> TypeResult<()> {
        {
            // Borrow the root slot
            let slot = self
                .0
                .get_mut(&lval.ident)
                .ok_or_else(|| Error::UnknownVar(lval.ident.clone()))?;
            let mut t: &mut Type = &mut slot.tipe;
            let mut remaining = lval.derefs;

            // Consume all the `*` (Box or &mut)
            while remaining > 0 {
                match t {
                    // Box<T> — just peel one level
                    Type::Box(inner) => {
                        t = inner.as_mut();
                        remaining -= 1;
                    }
                    // &mut U — recurse through after ending this borrow
                    Type::Ref(inner_lval, is_mut) => {
                        if !*is_mut {
                            return Err(Error::UpdateBehindImmRef(lval.clone()));
                        }
                        let next = inner_lval.clone();
                        // Drop &mut borrow before recursing
                        let _ = t;
                        let _ = slot;

                        return self.write(&next, tipe);
                    }
                    _ => {
                        return Err(Error::UpdateBehindImmRef(lval.clone()));
                    }
                }
            }

            if let Type::Ref(inner_lval, is_mut) = t {
                if !*is_mut {
                    return Err(Error::UpdateBehindImmRef(lval.clone()));
                }
                let next = inner_lval.clone();
                let _ = t;
                let _ = slot;
                return self.write(&next, tipe);
            }

            *t = tipe;
        }
        Ok(())
    }

    pub fn drop(&mut self, l: Lifetime) {
        self.0.retain(|_, slot| slot.lifetime != l);
    }
}

impl Slot {
    pub fn new(tipe: Type, lifetime: Lifetime) -> Self {
        Slot { tipe, lifetime }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Context {
    pub env: Env,
    pub lifetime_stack: Vec<Lifetime>,
}

impl Context {
    // l ≥ m, the ordering relation on liftimes (Note (2) pg. 13)
    fn lifetime_contains(&self, l: Lifetime, m: Lifetime) -> bool {
        let mut found_l = false;
        for lt in self.lifetime_stack.iter().rev() {
            if *lt == m {
                return found_l;
            }
            if *lt == l {
                found_l = true;
            }
        }
        false
    }

    // Γ ⊢ T ≥ l (Definition 3.21)
    fn well_formed(&self, tipe: &Type, l: Lifetime) -> bool {
        match tipe {
            Type::Unit | Type::Int => true,
            Type::Box(inner) | Type::Undefined(inner) => self.well_formed(inner, l),
            Type::Ref(_, _) => self.lifetime_contains(l.clone(), l),
        }
    }

    pub fn type_expr(&mut self, expr: &mut Expr) -> TypeResult<Type> {
        use crate::utils::Expr;
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Unit => Ok(Type::Unit),

            Expr::Lval(lval, _) => {
                if self.env.read_prohibited(lval) {
                    return Err(Error::MovedOut(lval.clone()));
                }

                let slot = self.env.type_lval(lval)?;
                let is_copyable = matches!(slot.tipe, Type::Int | Type::Unit);

                if is_copyable {
                    expr.make_copyable();
                    Ok(slot.tipe)
                } else {
                    self.env.moove(lval)?;
                    Ok(slot.tipe)
                }
            }

            _ => todo!(), //still need to implement for Boxed, Ref, Let, Assign, Drop
        }
    }

    pub fn type_stmt(&mut self, stmt: &mut Stmt) -> TypeResult<()> {
        use crate::utils::Stmt::*;

        match stmt {
            LetMut(var, rhs) => {
                let rhs_type = self.type_expr(rhs)?;
                let declared_type = rhs_type.clone(); // assume declared type = inferred

                let lifetime = self.fresh_lifetime();
                self.env.insert(var, declared_type, lifetime.clone());
                self.lifetime_stack.push(lifetime.clone());

                // No inner body in LetMut, so just pop and drop
                self.lifetime_stack.pop();
                self.env.drop(lifetime);

                Ok(())
            }

            Assign(lval, expr) => {
                let rhs_type = self.type_expr(expr)?;
                self.env.write(lval, rhs_type)?;
                Ok(())
            }

            Expr(expr) => {
                self.type_expr(expr)?;
                Ok(())
            }
        }
    }

    pub fn fresh_lifetime(&self) -> Lifetime {
        Lifetime(self.lifetime_stack.len())
    }
}

impl Expr {
    pub fn make_copyable(&mut self) {
        if let Expr::Lval(_, ref mut copyable) = self {
            *copyable = true;
        }
    }
}
