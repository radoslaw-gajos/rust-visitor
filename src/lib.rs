use std::any::Any;
use std::cell::RefCell;
use core::f64::consts::PI;

trait ShapeVisitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_square(&self, square: &Square);
    fn as_any(&self) -> &dyn Any;
}

struct AreaVisitor {
    area: RefCell<f64>,
}

impl AreaVisitor {
    fn new() -> Box<dyn ShapeVisitor> {
        Box::new(AreaVisitor {
            area: RefCell::new(0.0),
        })
    }
}

impl ShapeVisitor for AreaVisitor {
    fn visit_circle(&self, circle: &Circle) {
        let area = 2.0 * *circle.radius.borrow() * PI;
        *self.area.borrow_mut() += area;
    }
    fn visit_square(&self, square: &Square) {
        let x = *square.side_length.borrow();
        let area = x * x;
        *self.area.borrow_mut() += area;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct PerimeterVisitor {
    perimeter: RefCell<f64>,
}

impl PerimeterVisitor {
    fn new() -> Box<dyn ShapeVisitor> {
        Box::new(PerimeterVisitor {
            perimeter: RefCell::new(0.0),
        })
    }
}

impl ShapeVisitor for PerimeterVisitor {
    fn visit_circle(&self, circle: &Circle) {
        let radius = *circle.radius.borrow();
        let perimeter = PI * radius * radius;
        *self.perimeter.borrow_mut() += perimeter;
    }
    fn visit_square(&self, square: &Square) {
        let side_length = *square.side_length.borrow();
        let perimeter = side_length * 4.0;
        *self.perimeter.borrow_mut() += perimeter;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

trait Shape {
    fn accept(&self, visitor: &dyn ShapeVisitor);
}

struct Circle {
    radius: RefCell<f64>,
}

impl Circle {
    fn new(radius: f64) -> Box<dyn Shape> {
        Box::new(Circle {
            radius: RefCell::new(radius),
        })
    }
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn ShapeVisitor) {
        visitor.visit_circle(self);
    }
}

struct Square {
    side_length: RefCell<f64>,
}

impl Square {
    fn new(side_length: f64) -> Box<dyn Shape> {
        Box::new(Square {
            side_length: RefCell::new(side_length),
        })
    }
}

impl Shape for Square {
    fn accept(&self, visitor: &dyn ShapeVisitor) {
        visitor.visit_square(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_the_area_of_a_circle() {
        // given
        let radius = 1.0;
        let circle = Circle::new(radius);
        let visitor = AreaVisitor::new();

        // when
        circle.accept(visitor.as_ref());
        let area_visitor = visitor.as_any().downcast_ref::<AreaVisitor>().unwrap();
        let area = *area_visitor.area.borrow();

        // then
        assert_eq!(area, 2.0 * PI);
    }

    #[test]
    fn should_calculate_the_area_of_a_square() {
        // given
        let side_length = 1.0;
        let square = Square::new(side_length);
        let visitor = AreaVisitor::new();

        // when
        square.accept(visitor.as_ref());
        let area_visitor = visitor.as_any().downcast_ref::<AreaVisitor>().unwrap();
        let area = *area_visitor.area.borrow();

        // then
        assert_eq!(area, 1.0);
    }

    #[test]
    fn should_calculate_the_perimeter_of_a_circle() {
        // given
        let radius = 1.0;
        let circle = Circle::new(radius);
        let visitor = PerimeterVisitor::new();

        // when
        circle.accept(visitor.as_ref());
        let perimeter_visitor = visitor.as_any().downcast_ref::<PerimeterVisitor>().unwrap();
        let perimeter = *perimeter_visitor.perimeter.borrow();

        // then
        assert_eq!(perimeter, PI);
    }

    #[test]
    fn should_calculate_the_perimeter_of_a_square() {
        // given
        let side_length = 1.0;
        let square = Square::new(side_length);
        let visitor = PerimeterVisitor::new();

        // when
        square.accept(visitor.as_ref());
        let perimeter_visitor = visitor.as_any().downcast_ref::<PerimeterVisitor>().unwrap();
        let perimeter = *perimeter_visitor.perimeter.borrow();

        // then
        assert_eq!(perimeter, 4.0);
    }
}
