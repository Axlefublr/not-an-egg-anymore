use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let stack = GlobalStack(Arc::new(Mutex::new(VecDeque::from([82]))));
    let stack2 = stack.clone();
    stack.push(93);
    stack2.push(23);
}

struct GlobalStack<T>(Arc<Mutex<VecDeque<T>>>);

impl<T> GlobalStack<T> {
    pub fn push(&self, value: T) {
        let mut vector = self.0.lock().unwrap();
        vector.push_back(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut vector = self.0.lock().unwrap();
        vector.pop_back()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}