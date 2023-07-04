pub trait MyIterator {
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>;
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl MyIterator for Counter {
    // associated type
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() consumes the vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // v1_iter is an iterator

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // map() is an iterator adaptor
    assert_eq!(v1, vec![1, 2, 3]); // v1 is still valid
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // iter() returns an iterator over immutable references
    // to the items in a collection
    let mut v1_iter = v1.iter();

    // next() returns the next value in the iterator
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn fliters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ]
    );
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    // next() is called directly
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));

    // next() returns None after the iterator has returned all the items
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // zip() creates an iterator that combines elements from two iterators
        .map(|(a, b)| a * b) // map() creates an iterator that calls a closure on each element
        .filter(|x| x % 3 == 0) // filter() creates an iterator that uses a closure to decide what to keep
        .sum(); // sum() consumes the iterator and returns the sum of the items

    assert_eq!(18, sum);
}
