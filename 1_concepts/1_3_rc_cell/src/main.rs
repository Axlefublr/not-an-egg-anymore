use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let stack = GlobalStack(Arc::new(Mutex::new(vec![82])));
    let stack2 = stack.clone();
    stack.push(93);
    stack2.push(23);
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

    pub fn peek(&self) -> Option<&T> {
        let vector = self.0.lock().unwrap();
        // Это конечно же не работает
        vector.get(vector.len())
    }

    pub fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}