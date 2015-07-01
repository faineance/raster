use canvas::{Canvas, Colour};
use std::fmt::Debug;
pub trait Drawable: Debug {
	fn draw(&self, Colour, &mut Canvas);
}