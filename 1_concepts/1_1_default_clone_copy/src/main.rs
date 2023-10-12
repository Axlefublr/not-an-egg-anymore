use nonempty::{NonEmpty, nonempty};

fn main() {
    let poly = Polyline::new(nonempty![Point {x: 0, y: 0}, Point { x: -1, y: 7 }]);
}

#[derive(Clone, Copy, Default)]
struct Point {
    pub x: i32, // координаты, возможно, могут быть негативными
    pub y: i32,
}

#[derive(Clone)]
struct Polyline(Vec<Point>);

impl Polyline {
    pub fn new(points: NonEmpty<Point>) -> Self {
        Self(points.into())
    }

    pub fn get(&self, index: usize) -> Option<&Point> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Point> {
        self.0.get_mut(index)
    }
}