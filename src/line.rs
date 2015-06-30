use point::Point2D;

pub struct Line2D<T>
{
    a: Point2D<T>,
    b: Point2D<T>
}

impl<T> Line2D<T> {
	pub fn new(a: Point2D<T>, b: Point2D<T>) -> Line2D<T> {
		Line2D {
			a: a, b: b
		}
	}
}