struct Point {
  x: f64,
  y: f64,
}

struct Line {
  start: Point,
  end: Point,
}

impl Line {
  fn len(&self) -> f64 {
    let dx = self.start.x - self.end.x;
    let dy = self.start.x - self.end.y;
    (dx * dx + dy * dy).sqrt();
  }
}
