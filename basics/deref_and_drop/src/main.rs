use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

// The Drop trait requires us to implement one method named drop that takes a mutable reference to self.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // The Rust compiler inserts calls to the drop function automatically at the end of the curly brackets for you.
    // The main function is a little special, and the compiler doesn’t let us call drop explicitly in main.
    // We’re not allowed to call drop explicitly because Rust would still automatically call drop on the values at the end of main.
    // This would be a double free error because Rust would be trying to clean up the same value twice.
    // Double frees are memory corruption bugs that don’t always manifest in an obvious way, so Rust protects you from them.

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // e.drop(); // error: explicit use of destructor method
    drop(e); // explicit call to drop
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
