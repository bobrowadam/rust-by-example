pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

#[derive(Eq, Debug)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl<'a> PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        (self.p1 == other.p1) && (self.p2 == other.p2)
    }
}

#[derive(Debug, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

pub fn rect_area(r: Rectangle) -> isize {
    let height = r.p1.x - r.p2.x;
    let width = r.p1.y - r.p2.y;
    width * height
}

pub fn square(p: Point, length: isize) -> Rectangle {
    let p2_height = p.x + length;
    let p2_width = p.y + length;
    let p1 = p;
    let p2 = Point {
        x: p2_height,
        y: p2_width,
    };

    Rectangle { p1: p1, p2: p2 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_custom_types() {
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
        assert_eq!(peter.name, "Peter");
    }

    #[test]
    fn calc_rectangle_area() {
        let p1 = Point { x: -1, y: -1 };
        let p2 = Point { x: 11, y: 11 };
        assert_eq!(rect_area(Rectangle { p1, p2 }), 144);
    }
    #[test]
    fn gen_rectangle_from_point_and_length() {
        let rec1 = square(Point { x: 1, y: 1 }, 10);
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 11, y: 11 };
        assert_eq!(rec1, Rectangle { p1, p2 });
    }
}
