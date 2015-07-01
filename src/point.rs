use std::ops::{Add, Sub, Neg};
use num::{ToPrimitive};
use num::Num;
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