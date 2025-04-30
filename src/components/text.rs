use macroquad::prelude::*;
use crate::components::DrawableComponent;

pub struct TextComponent<'a> {
    pub text: &'a str,
    pub font_size: f32,
    pub color: Color,
    pub center: bool,
    pub x: f32,
    pub y: f32,
}

impl<'a> TextComponent<'a> {
    pub fn new(text: &'a str, font_size: f32) -> Self {
        Self {
            text,
            font_size,
            color: WHITE,
            center: true,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn align_center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }
}

impl<'a> DrawableComponent for TextComponent<'a> {
    fn draw(&mut self) {
        let measured = measure_text(self.text, None, self.font_size as u16, 1.0);

        let x = if self.center {
            self.x - measured.width / 2.0
        } else {
            self.x
        };

        draw_text(self.text, x, self.y, self.font_size, self.color);
    }
}
