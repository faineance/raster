use num::{FromPrimitive, Num, Zero, One};
use std::ops::{Sub};
use point::{Point2D, orient2d};
use drawable::Drawable;
use canvas::{Canvas, Colour};
use std::cmp::{min, max, Ord};

pub struct Triangle2D<T>
{
	a: Point2D<T>,
	b: Point2D<T>,
	c: Point2D<T>,
}

impl<T> Triangle2D<T> where T: Num + Copy {
	pub fn new(a: Point2D<T>, b: Point2D<T>, c: Point2D<T>) -> Triangle2D<T> {
		Triangle2D {
			a: a, b: b, c: c
		}
	}
}

impl<T> Drawable for Triangle2D<T> where T: Num + Sub + FromPrimitive + Ord + Copy {
	fn draw(&self, colour: Colour, canvas: &mut Canvas) {
		// Compute bounding box
		let mut min_x = min(min(self.a.x, self.b.x), self.c.x);
		let mut min_y = min(min(self.a.y, self.b.y), self.c.y);
		let mut max_x = max(max(self.a.x, self.b.x), self.c.x);
		let mut max_y = max(max(self.a.y, self.b.y), self.c.y);

		// clip against canvas bounds
		min_x = max(min_x, Zero::zero());
		min_y = max(min_y, Zero::zero());

		max_x = min(max_x, FromPrimitive::from_usize(canvas.width).unwrap());
		max_y = min(max_y, FromPrimitive::from_usize(canvas.height).unwrap());

		// triangle setup
		let aab = self.a.y - self.b.y;
		let bab = self.b.x - self.a.x;

		let abc = self.b.y - self.c.y;
		let bbc = self.c.x - self.b.x;

		let aca = self.a.y - self.c.y;
		let bca = self.c.x - self.a.x;

		// barycentric coordinates at min_x/min_y corner
		let mut p = Point2D::new(min_x, min_y);

		let mut w0_row = orient2d(self.b, self.c, p); //fbc
		let mut w1_row = orient2d(self.c, self.a, p); //fca
		let mut w2_row = orient2d(self.a, self.b, p); //fab

		while  p.y <= max_y {
			let mut w0 = w0_row;
			let mut w1 = w1_row;
			let mut w2 = w2_row;
			while p.x <= max_x {

				// If p is on or inside edges, place pixel.
				if w0 >= Zero::zero() && w1 >= Zero::zero() && w2 >= Zero::zero() {
					// empty
					
				}
				// Move to the right
				let mut w0 = w0 + abc;
				let mut w1 = w1 + aca;
				let mut w2 = w2 + aab;
				p.y = p.y + One::one();
			}
			// One row step
			w0_row = w0_row + bbc;
			w1_row = w1_row + bca;
			w2_row = w2_row + bab;
			p.y = p.y + One::one();
		}

	}
}