fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Semigroup and Monoid Tests
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


    // Functor Tests
    #[test]
    fn option_fmap() {
        let expected = Some(100).fmap(|x| x + 1);
        assert_eq!(expected, Some(101));
    }

    
    #[test]
    fn option_funzip() {
        let expected =  Some((vec![1, 2, 3], 4)).funzip();
        assert_eq!(expected.0, Some(vec![1, 2, 3]));
        assert_eq!(expected.1, Some(4))
    }
    /*
    #[test]
    fn option_functor_none() {
        let none : Option<(i32, i32)> = None;
        assert_eq!(none.fmap(|p| p.0 + p.1), None);
        assert_eq!(none.funzip(), (None, None))
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


trait Monoid<O>: Semigroup<O>
where
    O: BinOp<Self>,
    Self: Sized,
{
    fn identity() -> Self;

    fn mconcat<I: Iterator<Item = Self>>(iter: I) -> Self {
        <Self as Semigroup<O>>::mconcat(Self::identity(), iter)
    }
}
     

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

impl BinOp<String> for Add {
    fn op(lhs: String, rhs: String) -> String {
        let mut res = String::from(lhs);
        res.push_str(&rhs);
        res
    }
}

// Semigroup
impl Semigroup<Add> for i32 {}
impl Semigroup<Mul> for i32 {}
impl Semigroup<Add> for String {}

// Monoid
impl Monoid<Add> for i32 {
    fn identity() -> Self {
        0
    }
}
impl Monoid<Mul> for i32 {
    fn identity() -> Self {
        1
    }
}
impl Monoid<Add> for String {
    fn identity() -> Self {
        "".to_string()
    }
}

 

// Functor Traits
trait FunctorTypes {
    type Inner;
    type Outer<T>;
}

// Example for Option
impl<T> FunctorTypes for Option<T> {
    type Inner = T;
    type Outer<S> = Option<S>;
}

trait Functor: FunctorTypes {
    fn fmap<T>(self, f: impl Fn(Self::Inner) -> T) -> Self::Outer<T>;

    fn funzip<A, B>(self) -> (Self::Outer<A>, Self::Outer<B>)
    where
        Self: Clone,                      
        Self::Inner: Clone + Into<(A, B)> 
    {
        (
            self.clone().fmap(|p| {
                let (a, _b) = p.clone().into();
                a
            }),
            self.fmap(|p| {
                let (_a, b) = p.into();
                b
            }),
        )
    }
}

// Example for Option
impl<S> Functor for Option<S> {
    fn fmap<T>(self, f: impl Fn(S) -> T) -> Option<T> {
        match self {
            None => None,
            Some(x) => Some(f(x)),
        }
    }
}

