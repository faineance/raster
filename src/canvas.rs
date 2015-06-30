use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io;

#[derive(Copy, Clone)]
pub struct Colour {
	r: u8,
	g: u8,
	b: u8,
}

pub struct Canvas {
	data: Vec<Vec<Colour>>,
	width: usize,
	height: usize,
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

	pub fn set(&mut self, x: usize, y: usize, colour: Colour) {
		self.data[y][x] = colour;
	}

	pub fn get(&self, x: usize, y: usize) -> Colour {
		self.data[y][x]
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
}