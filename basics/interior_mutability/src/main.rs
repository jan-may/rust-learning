pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, // messenger is a reference to a Messenger trait object
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // LimitTracker::new takes a reference to a Messenger trait object
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger, // shorthand for messenger: messenger
            value: 0,
            max,
        }
    }

    // LimitTracker::set_value takes a mutable reference to self
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        // self.messenger.send(&self.value.to_string()); // this won't compile
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            // self.messenger.send("Error: You are over your quota!"); // this won't compile
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            // self.messenger.send("Urgent warning: You've used up over 90% of your quota!"); // this won't compile
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            // self.messenger.send("Warning: You've used up over 75% of your quota!"); // this won't compile
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // MockMessenger implements the Messenger trait
    struct MockMessenger {
        // sent_messages is a vector of strings
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        // MockMessenger::new returns a MockMessenger instance
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // MockMessenger::send takes a reference to self and a string slice
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));

            // push the message to the sent_messages vector
            // self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // create a MockMessenger instance
        let mock_messenger = MockMessenger::new();

        // create a mutable LimitTracker instance
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // set the value to 80
        limit_tracker.set_value(80);

        // assert that the first message in the sent_messages vector is "Warning: You've used up over 75% of your quota!"
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
