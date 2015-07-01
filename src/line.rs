use point::Point2D;
use std::ops::{Add, Sub, Neg};
use num::{Zero, One, Num, ToPrimitive};
use drawable::Drawable;
use canvas::{Canvas, Colour};
pub struct Line2D<T>
{
	a: Point2D<T>,
	b: Point2D<T>,
}

impl<T> Line2D<T> {
	pub fn new(a: Point2D<T>, b: Point2D<T>) -> Line2D<T> {
		Line2D {
			a: a, b: b
		}
	}
}

impl<T> Drawable for Line2D<T> where T: Sub<T, Output=T> + Add<T, Output=T> + Copy + PartialOrd + Zero + One + Num + ToPrimitive {
	fn draw(&self, colour: Colour, canvas: &mut Canvas) {
		let dx  = self.b.x - self.a.x;
		let dy  = self.b.y - self.a.y;
		let de  = dy + dy;
		let dne = (dy - dx) + (dy - dx);
		let mut d   = dy + dy - dx;
		let mut x = self.a.x;
		let mut y = self.a.y;
		while x < self.b.x {
			if d <= Zero::zero() {
				d = d + de;
				x = x + One::one();
			} else {
				d = d + dne;
				x = x + One::one();
				y = y + One::one();
			}
			canvas.set(x,y, colour)
		}
	}
}