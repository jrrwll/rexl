use List::{Cons, Nil};
use std::cell::RefCell;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

#[test]
#[allow(unused_variables)]
fn test() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    // Deref trait, *t
    // Drop trait
    assert_eq!(5, *y);

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = MyBox::new(String::from("Rust"));
    println!("SmartPointer created.");
    //hello(&(*m)[..]);
    hello(&m);
    drop(m);
    println!("SmartPointer dropped before the end of main.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    // RcList
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcList::Cons(Rc::clone(&value), Rc::new(RcList::Nil)));

    let b = RcList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RcList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("SmartPointer is being dropped before the end of main.");
        drop(&self.0)
    }
}
// T: Deref<Target=U>,    &T -> &U
// T: DerefMut<Target=U>, &mut T -> &mut U
// T: Deref<Target=U>,    &mut T to &U

#[derive(Debug)]
enum RcList {
    Cons(Rc<RefCell<i32>>, Rc<RcList>),
    Nil,
}

