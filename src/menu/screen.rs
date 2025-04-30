use macroquad::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::GameState;
use crate::components::button::Button;
use crate::components::text::TextComponent;
use crate::components::layout::Column;

pub struct MenuScreen<'a> {
    layout: Column<'a>,
    state_transition: Rc<RefCell<Option<GameState>>>,
}

impl<'a> MenuScreen<'a> {
    pub fn new() -> Self {
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let state_transition = Rc::new(RefCell::new(None));
        let state_transition_clone = Rc::clone(&state_transition);

        let title = TextComponent::new("Welcome to Metal Against Demons!", 50.0)
            .color(WHITE)
            .align_center(true)
            .at(screen_center.x, screen_center.y - 100.0);

        let start_button = Button::builder(
            screen_center.x - 100.0,
            screen_center.y,
            200.0,
            60.0,
            "Start Game",
        )
            .on_click(move || {
                *state_transition_clone.borrow_mut() = Some(GameState::Playing);
            })
            .color(Color::from_rgba(90, 20, 20, 255))
            .hover_color(Color::from_rgba(60, 20, 20, 255))
            .build();

        let layout = Column::new()
            .centered()
            .spacing(20.0)
            .add_child(Box::new(title))
            .add_child(Box::new(start_button));

        Self {
            layout,
            state_transition,
        }
    }

    pub fn draw(&mut self) -> Option<GameState> {
        clear_background(Color::from_rgba(30, 30, 30, 255));        
        self.layout.draw();
        self.state_transition.borrow_mut().take()
    }
}
