fn main() {
    let poly = Polyline::new(vec![Point::default(), Point { x: 3, y: 5}]);
}

#[derive(Clone, Copy, Default)]
struct Point {
    pub x: i32, // координаты, возможно, могут быть негативными
    pub y: i32,
}

#[derive(Clone)]
struct Polyline(Vec<Point>);

impl Polyline {
    /// # Panics
    ///
    /// If `points` is empty
    pub fn new(points: Vec<Point>) -> Self {
        assert!(!points.is_empty());
        Self(points)
    }

    pub fn get(&self) -> &Vec<Point> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Point> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn panics_if_empty() {
        Polyline::new(Vec::new());
    }
}