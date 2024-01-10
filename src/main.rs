use std::cell::RefCell;
use std::rc::Rc;

struct SharedCounter(Rc<RefCell<u8>>);

fn reassign(source: SharedCounter, dest: SharedCounter) {
    assert!(*source.0.borrow() > 0);
    *source.0.borrow_mut() -= 1;
    assert!(*dest.0.borrow() < u8::MAX);
    *dest.0.borrow_mut() += 1;
}

fn main() {
    let counter = Rc::new(RefCell::new(15));
    let source = SharedCounter(Rc::clone(&counter));
    let dest = SharedCounter(Rc::clone(&counter));
    reassign(source, dest);
    println!("{}", counter.borrow());
    // counter dropped here
    println!("done");
}
