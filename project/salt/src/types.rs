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

impl Slot {
    /// Tests expect `Slot::new(ty, lt)`
    pub fn new(tipe: Type, lifetime: Lifetime) -> Self {
        Slot { tipe, lifetime }
    }
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
        let mut slot = self
            .0
            .get(&lval.ident)
            .ok_or_else(|| Error::UnknownVar(lval.ident.clone()))?
            .clone();
        for _ in 0..lval.derefs {
            slot = match slot.tipe {
                Type::Box(inner) => {
                    if let Type::Undefined(_) = *inner {
                        return Err(Error::MovedOut(lval.clone()));
                    }
                    Slot {
                        tipe: *inner,
                        lifetime: slot.lifetime,
                    }
                }
                Type::Ref(ref inner, _) => self.type_lval(inner)?,
                other => return Err(Error::CannotDeref(other)),
            };
        }
        Ok(slot)
    }

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

    pub fn moove(&mut self, lval: &Lval) -> TypeResult<()> {
        let mut slot = self
            .0
            .get_mut(&lval.ident)
            .ok_or_else(|| Error::UnknownVar(lval.ident.clone()))?;
        let mut t: &mut Type = &mut slot.tipe;
        for _ in 0..lval.derefs {
            match t {
                Type::Box(inner) => t = inner.as_mut(),
                _ => return Err(Error::MoveBehindRef(lval.clone())),
            }
        }
        if let Type::Ref(_, _) = t {
            return Err(Error::MoveBehindRef(lval.clone()));
        }
        *t = Type::Undefined(Box::new(t.clone()));
        Ok(())
    }

    pub fn muut(&self, lval: &Lval) -> bool {
        let mut t = match self.0.get(&lval.ident) {
            Some(s) => s.tipe.clone(),
            None => return false,
        };
        let mut rem = lval.derefs;
        loop {
            if rem > 0 {
                match t {
                    Type::Box(inner) => {
                        t = *inner;
                        rem -= 1;
                    }
                    Type::Ref(inner, is_mut) => {
                        if !is_mut {
                            return false;
                        }
                        if let Some(s) = self.0.get(&inner.ident) {
                            t = s.tipe.clone();
                            rem = inner.derefs;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            } else {
                match t {
                    Type::Box(inner) => t = *inner,
                    Type::Ref(inner, is_mut) => {
                        if !is_mut {
                            return false;
                        }
                        if let Some(s) = self.0.get(&inner.ident) {
                            t = s.tipe.clone();
                            rem = inner.derefs;
                        } else {
                            return false;
                        }
                    }
                    _ => return true,
                }
            }
        }
    }

    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::Undefined(a), _) => self.compatible(a, t2),
            (_, Type::Undefined(b)) => self.compatible(t1, b),
            (Type::Int, Type::Int) | (Type::Unit, Type::Unit) => true,
            (Type::Box(a), Type::Box(b)) => self.compatible(a, b),
            (Type::Ref(_, m1), Type::Ref(_, m2)) => m1 == m2,
            _ => false,
        }
    }

    pub fn write(&mut self, lval: &Lval, new_t: Type) -> TypeResult<()> {
        use Error::*;

        // 1) forbid assignment if there’s an outstanding borrow on the base ident
        for slot in self.0.values() {
            if let Type::Ref(ref tgt, _) = slot.tipe {
                if tgt.ident == lval.ident {
                    return Err(AssignAfterBorrow(lval.clone()));
                }
            }
        }

        // 2) flatten any &mut chains into `flat`
        let mut flat = lval.clone();
        while let Some(sr) = self.0.get(&flat.ident) {
            if let Type::Ref(inner, true) = &sr.tipe {
                flat.ident = inner.ident.clone();
                flat.derefs = inner.derefs + (flat.derefs - 1);
            } else {
                break;
            }
        }

        // 3) immutably borrow once to extract `old_t`
        let old_t = {
            let sr = self
                .0
                .get(&flat.ident)
                .ok_or_else(|| UnknownVar(flat.ident.clone()))?;
            let mut t = sr.tipe.clone();
            for _ in 0..flat.derefs {
                t = match t {
                    Type::Box(inner) => *inner,
                    _ => return Err(UpdateBehindImmRef(lval.clone())),
                };
            }
            t
        };

        // 4) forbid writing through an immutable ref
        if let Type::Ref(_, false) = old_t {
            return Err(UpdateBehindImmRef(lval.clone()));
        }
        // 5) compatibility check
        if !self.compatible(&old_t, &new_t) {
            return Err(IncompatibleTypes(old_t.clone(), new_t.clone()));
        }

        // 6) now mutably borrow and apply the write
        let slot_entry = self
            .0
            .get_mut(&flat.ident)
            .ok_or_else(|| UnknownVar(flat.ident.clone()))?;
        let mut t_ptr: &mut Type = &mut slot_entry.tipe;
        for _ in 0..flat.derefs {
            if let Type::Box(inner) = t_ptr {
                t_ptr = inner.as_mut();
            } else {
                unreachable!();
            }
        }
        *t_ptr = new_t;
        Ok(())
    }

    pub fn drop(&mut self, l: Lifetime) {
        self.0.retain(|_, slot| slot.lifetime != l);
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Context {
    pub env: Env,
    pub lifetime_stack: Vec<Lifetime>,
}

impl Context {
    fn lifetime_contains(&self, l: Lifetime, m: Lifetime) -> bool {
        let mut found = false;
        for lt in self.lifetime_stack.iter().rev() {
            if *lt == m {
                return found;
            }
            if *lt == l {
                found = true;
            }
        }
        false
    }

    fn well_formed(&self, tipe: &Type, l: Lifetime) -> bool {
        match tipe {
            Type::Unit | Type::Int => true,
            Type::Box(inner) | Type::Undefined(inner) => self.well_formed(inner, l.clone()),
            Type::Ref(_, _) => self.lifetime_contains(l.clone(), l),
        }
    }

    pub fn type_expr(&mut self, expr: &mut Expr) -> TypeResult<Type> {
        use Expr::*;
        match expr {
            Int(_) => Ok(Type::Int),
            Unit => Ok(Type::Unit),
            Lval(lv, _) => {
                let slot = self.env.type_lval(lv)?;
                // detect moved-out
                fn has_undef(ty: &Type) -> bool {
                    match ty {
                        Type::Undefined(_) => true,
                        Type::Box(inner) => has_undef(inner),
                        _ => false,
                    }
                }
                if has_undef(&slot.tipe) {
                    return Err(Error::MovedOut(lv.clone()));
                }
                let is_copy = matches!(slot.tipe, Type::Int | Type::Unit);
                for other in self.env.0.values() {
                    if let Type::Ref(_, mutbl) = &other.tipe {
                        if *mutbl && is_copy {
                            return Err(Error::CopyAfterMutBorrow(lv.clone()));
                        }
                        if !*mutbl && !is_copy {
                            return Err(Error::MoveAfterBorrow(lv.clone()));
                        }
                    }
                }
                if is_copy {
                    expr.make_copyable();
                    Ok(slot.tipe)
                } else {
                    self.env.moove(lv)?;
                    Ok(slot.tipe)
                }
            }
            Box(inner) => Ok(Type::boxx(self.type_expr(inner)?)),
            Borrow(lv, is_mut) => {
                let slot = self.env.type_lval(lv)?;
                if let Type::Undefined(_) = slot.tipe {
                    return Err(Error::MovedOut(lv.clone()));
                }
                if let Some(block_lt) = self.lifetime_stack.last() {
                    // use clone(), Lifetime isn’t Copy
                    if !self.lifetime_contains(slot.lifetime.clone(), block_lt.clone()) {
                        return Err(Error::LifetimeTooShort(Expr::Borrow(lv.clone(), *is_mut)));
                    }
                }
                if *is_mut {
                    if !self.env.muut(lv) {
                        return Err(Error::MutBorrowBehindImmRef(lv.clone()));
                    }
                    for other in self.env.0.values() {
                        if let Type::Ref(ref tgt, false) = &other.tipe {
                            if tgt.ident == lv.ident {
                                return Err(Error::MutBorrowAfterBorrow(lv.clone()));
                            }
                        }
                    }
                } else {
                    for other in self.env.0.values() {
                        if let Type::Ref(ref tgt, true) = &other.tipe {
                            if tgt.ident == lv.ident {
                                return Err(Error::BorrowAfterMutBorrow(lv.clone()));
                            }
                        }
                    }
                }
                Ok(Type::Ref(lv.clone(), *is_mut))
            }
            Block(stmts, final_e, lt) => {
                self.lifetime_stack.push(lt.clone());
                for s in stmts {
                    self.type_stmt(s)?;
                }
                let res = self.type_expr(final_e)?;
                let popped = self.lifetime_stack.pop().unwrap();
                self.env.drop(popped);
                Ok(res)
            }
        }
    }

    pub fn type_stmt(&mut self, stmt: &mut Stmt) -> TypeResult<()> {
        match stmt {
            Stmt::LetMut(var, rhs) => {
                if self.env.0.contains_key(var) {
                    return Err(Error::Shadowing(var.clone()));
                }
                let rhs_ty = self.type_expr(rhs)?;
                // detect moved-out initialiser
                if let Type::Undefined(_) = rhs_ty {
                    if let Expr::Lval(ref lv, _) = *rhs {
                        return Err(Error::MovedOut(lv.clone()));
                    }
                }
                self.env.insert(var, rhs_ty, self.fresh_lifetime());
                Ok(())
            }
            Stmt::Assign(lv, e) => {
                let rhs_ty = self.type_expr(e)?;
                self.env.write(lv, rhs_ty)?;
                Ok(())
            }
            Stmt::Expr(e) => {
                let _ = self.type_expr(e)?;
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
        if let Expr::Lval(_, c) = self {
            *c = true
        }
    }
}
