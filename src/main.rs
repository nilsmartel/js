use std::{cell::{Cell, RefCell}, ops::AddAssign, rc::Rc};
use dumpster::unsync::Gc;

pub mod parse;
mod interpreter;

fn main() {
    let x: Rc<Cell<i32>> = Rc::new(Cell::new(27));
    let y = x.clone();

    x.update(|x| x+1);
    x.update(|y| y+1);

    eprintln!("we expect x and y to be of value 29 now");
    dbg!(x);
    dbg!(y);

    let x: Gc<RefCell<i32>> = Gc::new(RefCell::new(27));
    let y = x.clone();
    *x.borrow_mut() += 1;
    *y.borrow_mut() += 1;
    eprintln!("we expect x and y to be of value 29 now");
    dbg!(x.borrow());
    dbg!(y.borrow());
}


/// NOTES:
/// get returns a reference to a value
/// e.g. get("a") => Reference, that we can even mutate or get the inner value
