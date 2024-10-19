trait ShapeVisitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_square(&self, square: &Square);
}

struct AreaVisitor {
}

struct PerimeterVisitor {
}

trait Shape {
    fn accept(&self, visitor: &dyn ShapeVisitor);
}

struct Circle {
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn ShapeVisitor) {
        visitor.visit_circle(self);
    }
}

struct Square {
}

impl Shape for Square {
    fn accept(&self, visitor: &dyn ShapeVisitor) {
        visitor.visit_square(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
