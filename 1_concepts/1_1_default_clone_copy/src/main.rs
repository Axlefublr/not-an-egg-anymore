fn main() {
    let poly = Polyline::build(vec![Point::default(), Point { x: 3, y: 5}]).unwrap();
}

#[derive(Clone, Copy, Default)]
struct Point {
    pub x: i32, // координаты, возможно, могут быть негативными
    pub y: i32,
}

#[derive(Clone)]
struct Polyline(Vec<Point>);

impl Polyline {
    pub fn build(points: Vec<Point>) -> Option<Self> {
        if points.is_empty() {
            None
        } else {
            Some(Self(points))
        }
    }

    pub fn get(&self, index: usize) -> Option<&Point> {
        self.0.get(index)
    }
}