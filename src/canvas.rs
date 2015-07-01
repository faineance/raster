use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io;
use num::{Num, ToPrimitive};
use drawable::Drawable;
#[derive(Copy, Clone)]
pub struct Colour {
	r: u8,
	g: u8,
	b: u8,
}
impl Colour {
	pub fn new(r: u8, g: u8, b: u8) -> Colour {
		Colour { r: r, g: g, b: b }
	}
}

pub struct Canvas {
	data: Vec<Vec<Colour>>,
	pub width: usize,
	pub height: usize,
}

impl Canvas {
	pub fn new(width: usize, height: usize, background: Colour) -> Canvas {
		let mut data: Vec<Vec<Colour>> = Vec::new();
		for _ in 0..height {
			let mut row: Vec<Colour> = Vec::new();
			for _ in 0..width {
				row.push(background);
			}
			data.push(row);
		}
		Canvas { data: data, width: width, height: height}
	}

	pub fn set<T: Num + ToPrimitive>(&mut self, x: T, y: T, colour: Colour) {
		self.data[y.to_usize().unwrap()][x.to_usize().unwrap()] = colour;
	}

	pub fn get<T: Num + ToPrimitive>(&self, x: T, y: T,) -> Colour {
		self.data[y.to_usize().unwrap()][x.to_usize().unwrap()]
	}

	pub fn write(&self, filename: &str) -> io::Result<()> {
		let path = Path::new(filename);
		let mut file = try!(File::create(&path));

		let header = format!("P6 {} {} 255\n", self.width, self.height);
		try!(file.write_all(header.as_bytes()));
		for x in 0..self.width {
			for y in 0..self.height {
				let colour = self.data[x][y];
				try!(file.write_all(format!("{} {} {}\n", colour.r, colour.g, colour.b).as_bytes()));
			}
		}
		Ok(())
	}
	pub fn draw<T: Drawable>(&mut self, shapes: Vec<T>, colour: Colour) {
		for shape in shapes.iter() {
			shape.draw(colour, self);
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use point::Point2D;
// 	use line::Line2D;
// 	#[test]
// 	fn drawline() {
// 		let background = Colour::new(0,0,0);
// 		let foreground = Colour::new(255,255,255);
// 		let mut canvas = Canvas::new(100, 100, foreground);
// 		let mut batch = Vec::new();
		

// 		batch.push(Line2D::new(Point2D::new(2, 2), Point2D::new(99, 99)));
// 		// canvas.draw(a , foreground);
// 		canvas.draw(batch , foreground);
// 		canvas.write("image.ppm");
// 	}
// }