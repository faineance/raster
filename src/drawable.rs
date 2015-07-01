use canvas::{Canvas, Colour};
pub trait Drawable {
	fn draw(&self, Colour, &mut Canvas);
}