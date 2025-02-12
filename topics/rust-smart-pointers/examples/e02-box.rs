use std::fmt;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::Cons(head, tail) => write!(f, "{} -> {}", head, tail),
            List::Nil => write!(f, "Nil"),
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{}", list);

    let b = Box::new(5);
    println!("b = {}", b);
}
