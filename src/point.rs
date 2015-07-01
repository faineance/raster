use std::ops::{Add, Sub, Neg};
use num::{ToPrimitive, Num};
use drawable::Drawable;
use canvas::{Canvas, Colour};

#[derive(Copy, Clone)]
pub struct Point2D<T> {
	pub x: T,
	pub y: T
}

impl<T: Num> Point2D<T> {
	pub fn new(x: T, y: T) -> Point2D<T> {
		Point2D {
			x: x, y: y
		}
	}
}

impl<T: Num + Add<T, Output=T>> Add for Point2D<T> {
	type Output = Point2D<T>;
	fn add(self, other: Point2D<T>) -> Point2D<T> {
		Point2D::new(self.x + other.x, self.y + other.y)
	}
}

impl<T: Num + Sub<T, Output=T>> Sub for Point2D<T> {
	type Output = Point2D<T>;
	fn sub(self, other: Point2D<T>) -> Point2D<T> {
		Point2D::new(self.x - other.x, self.y - other.y)
	}
}


impl<T: Num + Neg<Output=T>> Neg for Point2D<T> {
	type Output = Point2D<T>;
	fn neg(self) -> Point2D<T> {
		Point2D::new(-self.x, -self.y)
	}
}


impl<T: Num + ToPrimitive> Drawable for Point2D<T> where T: Copy{
	fn draw(&self, colour: Colour, canvas: &mut Canvas) {
		canvas.set(self.x, self.y, colour)
		
	}
}

pub fn orient2d<T: Num + Copy>(a: Point2D<T>, b: Point2D<T>, c: Point2D<T>) -> T {
	//return positive, if c to the left of   a->b.
    //return zero,     if c is colinear with a->b.
    //return negative, if c to the right of  a->b.
    let acx = a.x - c.x;
    let bcx = b.x - c.x;
    let acy = a.y - c.y;
    let bcy = b.y - c.y;
    acx * bcy - acy * bcx
}