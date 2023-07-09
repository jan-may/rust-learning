use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // List::tail returns a reference to the tail of the list
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // parent: RefCell<Rc<Node>>, // this will cause a memory leak
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        // *link.borrow_mut() = Rc::clone(&b); // this will panic
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail()); // Some(RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Nil }) }) })

    // Creating a Tree Data Structure: a Node with Child Nodes
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // branch has a strong_count of 1 and a weak_count of 0

    let branch = Rc::new(Node {
        value: 5,
        // parent: RefCell::new(Rc::clone(&leaf)), // this will cause a memory leak
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // *leaf.parent.borrow_mut() = Rc::clone(&branch); // this will cause a memory leak
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}
