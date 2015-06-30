use std::ops::{Add, Sub, Neg};

pub struct Point2D<T> {
	x: T,
	y: T
}

impl<T> Point2D<T> {
	pub fn new(x: T, y: T) -> Point2D<T> {
		Point2D {
			x: x, y: y
		}
	}
}

impl<T: Add<T, Output=T>> Add for Point2D<T> {
	type Output = Point2D<T>;
	fn add(self, other: Point2D<T>) -> Point2D<T> {
		Point2D::new(self.x + other.x, self.y + other.y)
	}
}

impl<T: Sub<T, Output=T>> Sub for Point2D<T> {
	type Output = Point2D<T>;
	fn sub(self, other: Point2D<T>) -> Point2D<T> {
		Point2D::new(self.x - other.x, self.y - other.y)
	}
}

impl<T: Neg<Output=T>> Neg for Point2D<T> {
	type Output = Point2D<T>;
	fn neg(self) -> Point2D<T> {
		Point2D::new(-self.x, -self.y)
	}
}
