fn main() {
    println!("Hello, world!");
}


trait BinOp<T> {
    fn op(lhs: T, rhs: T) -> T;
}

trait Semigroup<O> where
    O: BinOp<Self>,
    Self: Sized
{
    fn mconcat<I: Iterator<Item=Self>>(start: Self, iter: I) -> Self {
        let mut out = start;
        for item in iter {
            out = O::op(out, item)
        }
        out
    }
}

trait Monoid<O>: Semigroup<O> where
    O: BinOp<Self>, 
    Self: Sized
{
    fn mconcat<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut out =
    
    }
}

struct Add;
struct Mul;

impl BinOp<i32> for Add {
    fn op(lhs: i32, rhs: i32) -> i32 {
        lhs + rhs
    }
}

impl BinOp<i32> for Mul {
    fn op(lhs: i32, rhs: i32) -> i32 {
        lhs * rhs
    }
}

impl Semigroup<Add> for i32{}
impl Semigroup<Mul> for i32{}

