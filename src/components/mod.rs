pub mod button;
pub mod text;
pub mod layout;

pub trait DrawableComponent {
    fn draw(&mut self);
}