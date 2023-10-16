use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    println!("Implement me!");
}

struct GlobalStack<T>(Arc<Mutex<Vec<T>>>);

impl<T> GlobalStack<T> {
    pub fn push(&self, value: T) {
        let mut vector = self.0.lock().unwrap();
        vector.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut vector = self.0.lock().unwrap();
        vector.pop()
    }

    pub fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}