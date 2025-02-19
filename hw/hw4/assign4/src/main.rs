fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mconcat_semi_add_i32() {
        let expected = <i32 as Semigroup<Add>>::mconcat(10, vec![1, 2, 3, 4, 5].into_iter());
        assert_eq!(expected, 25)
    }

    #[test]
    fn mconcat_semi_mul_i32() {
        let expected = <i32 as Semigroup<Mul>>::mconcat(10, vec![1, 2, 3, 4, 5].into_iter());
        assert_eq!(expected, 1200)
    }
    /*
    #[test]
    fn mconcat_mon_add_i32() {
        let expected = <i32 as Monoid<Add>>::mconcat(vec![1, 2, 3, 4, 5].into_iter());
        assert_eq!(expected, 15)
    }

    #[test]
    fn mconcat_mon_mul_i32() {
        let expected = <i32 as Monoid<Mul>>::mconcat(vec![1, 2, 3, 4, 5].into_iter());
        assert_eq!(expected, 120)
    }

    #[test]
    fn mconcat_semi_str() {
        let s1 = String::from("abc");
        let s2 = String::from("def");
        let s3 = String::from("ghi");
        let expected = <String as Semigroup<Add>>::mconcat(s1, vec![s2, s3].into_iter());
        let actual = String::from("abcdefghi");
        assert_eq!(expected, actual);
    }

    #[test]
    fn mconcat_mon_str() {
        let s1 = String::from("abc");
        let s2 = String::from("def");
        let s3 = String::from("ghi");
        let expected = <String as Monoid<Add>>::mconcat(vec![s1, s2, s3].into_iter());
        let actual = String::from("abcdefghi");
        assert_eq!(expected, actual)
    }
     */
}

// Traits

trait BinOp<T> {
    fn op(lhs: T, rhs: T) -> T;
}

trait Semigroup<O>
where
    O: BinOp<Self>,
    Self: Sized,
{
    fn mconcat<I: Iterator<Item = Self>>(start: Self, iter: I) -> Self {
        let mut out = start;
        for item in iter {
            out = O::op(out, item)
        }
        out
    }
}

/*
trait Monoid<O>: Semigroup<O>
where
    O: BinOp<Self>,
    Self: Sized,
{
    fn mconcat<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut out = 0;
        for item in iter {
            out = O::op(out, item)
        }
        out
    }
}
     */

// Binop Implementations
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

// Semigroup
impl Semigroup<Add> for i32 {}
impl Semigroup<Mul> for i32 {}

/*/ Monoid
impl Monoid<Add> for i32 {}
impl Monoid<Mul> for i32 {}
*/
