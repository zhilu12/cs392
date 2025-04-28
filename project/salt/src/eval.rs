use crate::utils::{Expr, Ident, Lifetime, Lval, Stmt};
use std::collections::HashMap;

type Location = Ident;
type Owned = bool;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Int(i32),
    Ref(Location, Owned),
}

type Pvalue = Option<Value>;

#[derive(Debug)]
pub struct Slot {
    pub value: Pvalue,
    pub lifetime: Lifetime,
}

#[derive(Debug, Default)]
pub struct Store(pub HashMap<Location, Slot>);

impl Store {
    pub fn locate(&self, w: &Lval) -> Location {
        assert_eq!(w.derefs, 0, "Only base identifiers are supported in locate");
        w.ident.clone()
    }

    pub fn read(&self, x: &Lval) -> &Slot {
        let loc = self.locate(x);
        self.0
            .get(&loc)
            .expect("Attempted to read from unknown location")
    }

    pub fn write(&mut self, x: &Lval, v: Pvalue) -> Pvalue {
        assert_eq!(x.derefs, 0, "Only base identifiers are supported in write");
        let slot = self
            .0
            .get_mut(&x.ident)
            .expect("Attempted to write to unknown location");
        let old_val = slot.value.clone();
        slot.value = v;
        old_val
    }

    pub fn drop(&mut self, values: Vec<Pvalue>) {
        for pval in values {
            if let Some(Value::Ref(loc, _owned)) = pval {
                self.0.remove(&loc);
            }
        }
    }
}

pub struct Context {
    pub store: Store,
    pub counter: usize,
}
impl Context {
    pub fn eval_expr(&mut self, expr: &Expr, l: &Lifetime) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(*n),

            Expr::Unit => Value::Unit,

            Expr::Lval(lval, _copyable) => {
                let slot = self.store.read(lval);
                slot.value
                    .clone()
                    .expect("Attempted to read an uninitialized value")
            }

            Expr::Box(inner) => {
                let val = self.eval_expr(inner, l);
                let loc = self.fresh_location();

                self.store.0.insert(
                    loc.clone(),
                    Slot {
                        value: Some(val),
                        lifetime: l.clone(),
                    },
                );

                Value::Ref(loc, true)
            }

            Expr::Borrow(lval, _mutability) => {
                let loc = self.store.locate(lval).clone();
                Value::Ref(loc, false)
            }

            Expr::Block(stmts, final_expr, block_lifetime) => {
                let pre_keys: Vec<_> = self.store.0.keys().cloned().collect();

                for stmt in stmts {
                    self.eval_stmt(stmt, &block_lifetime.clone());
                }

                let result = self.eval_expr(final_expr, &block_lifetime.clone());

                let post_keys: Vec<_> = self.store.0.keys().cloned().collect();
                let new_keys: Vec<_> = post_keys
                    .into_iter()
                    .filter(|k| !pre_keys.contains(k))
                    .collect();

                let to_drop = new_keys
                    .into_iter()
                    .filter_map(|k| self.store.0.get(&k).map(|slot| slot.value.clone()))
                    .collect();

                self.store.drop(to_drop);

                result
            }
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt, l: &Lifetime) {
        match stmt {
            Stmt::LetMut(ident, expr) => {
                let val = self.eval_expr(expr, &l.clone());
                self.store.0.insert(
                    ident.clone(),
                    Slot {
                        value: Some(val),
                        lifetime: l.clone(),
                    },
                );
            }

            Stmt::Assign(lval, expr) => {
                let val = self.eval_expr(expr, &l.clone());
                self.store.write(lval, Some(val));
            }

            Stmt::Expr(expr) => {
                let _ = self.eval_expr(expr, &l.clone());
            }
        }
    }

    fn fresh_location(&mut self) -> String {
        let loc = format!("__box{}", self.counter);
        self.counter += 1;
        loc
    }
}
