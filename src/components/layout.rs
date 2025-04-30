// components/layout.rs

use macroquad::prelude::*;
use crate::components::DrawableComponent;

pub struct Column<'a> {
    children: Vec<Box<dyn DrawableComponent + 'a>>,
    center: bool,
    spacing: f32,
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            center: false,
            spacing: 10.0,
        }
    }

    pub fn centered(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn spacing(mut self, value: f32) -> Self {
        self.spacing = value;
        self
    }

    pub fn add_child(mut self, child: Box<dyn DrawableComponent + 'a>) -> Self {
        self.children.push(child);
        self
    }

    pub fn draw(&mut self) {
        for child in self.children.iter_mut() {
            // Aqui, você pode evoluir para passar a posição para os filhos.
            child.draw();
        }
    }
}
