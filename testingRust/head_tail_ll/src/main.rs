fn main() {
    println!("Hello, world!");
}

enum List<T> {
    Cons(T, Rc<RefCell<List<T>>>),
    Nil,
}

struct List<T> {
    head: List<T>,
}

impl List<T> {
    fn hd(&mut self) -> T {
        self
    }

    fn tl(&mut self) -> T {
        // mathc until the end?
        match self {
            some 
        }
    }

    fn get(&mut self, n: i32) -> T {

    }
}
