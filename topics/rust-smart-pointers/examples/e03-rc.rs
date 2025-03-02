use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    print_list(&b);
}

/*
How matching works in print_list function:
1. &**list: Dereference list to get the value of the Rc pointer, then dereference the value to get the value of the List enum.

  - list is a reference to a Rc pointer &Rc<List>.
  - *list is the value of the Rc pointer Rc<List>.
  - **list is the value of the List enum inside the Rc pointer.
  - &**list is a reference to the value of the List enum inside the Rc pointer.
*/
fn print_list(list: &Rc<List>) {
    match &**list {
        Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        Nil => (),
    }
}
