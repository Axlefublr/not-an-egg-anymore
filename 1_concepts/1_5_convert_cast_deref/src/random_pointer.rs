use std::ops::Deref;
use rand::Rng;

pub struct Random<T>(T, T, T);

impl<T> Random<T> {
    pub fn new(one: T, two: T, three: T) -> Self {
        Random(one, two, three)
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!("The range is from 0 to 2, inclusive. All of them are taken care of.")
        }
    }
}