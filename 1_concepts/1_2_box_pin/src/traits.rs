use std::fmt;
use std::ops;
use std::pin::Pin;

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: fmt::Debug> SayHi for T {}

struct Calc(i32);

impl Calc {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl MutMeSomehow for Wrapper<Calc> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // we mutate the value, but we don't move it
        let this = unsafe { self.get_unchecked_mut() };
        this.0.increment();
    }
}

impl<T> ops::Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
