pub type Ident = String;
pub type Copyable = bool;
pub type Mutable = bool;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime(pub usize);

impl Lifetime {
    pub fn global() -> Lifetime {
        Lifetime(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lval {
    pub ident: Ident,
    pub derefs: usize,
}

impl Lval {
    pub fn new(ident: &str, derefs: usize) -> Self {
        Lval {
            ident: ident.to_string(),
            derefs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unit,
    Int(i32),
    Lval(Lval, Copyable),
    Box(Box<Expr>),
    Borrow(Lval, Mutable),
    Block(Vec<Stmt>, Box<Expr>, Lifetime),
}

impl Expr {
    pub fn boxx(inner: Expr) -> Expr {
        Expr::Box(Box::new(inner))
    }

    pub fn block(stmts: Vec<Stmt>, final_expr: Expr, lifetime: Lifetime) -> Expr {
        Expr::Block(stmts, Box::new(final_expr), lifetime)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign(Lval, Expr),
    LetMut(Ident, Expr),
    Expr(Expr),
}
