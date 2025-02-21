use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

use smithay::{
    backend::input::KeyState,
    input::Seat,
    utils::{Logical, Point},
};

use crate::State;

#[derive(Debug, Default)]
pub struct SeatState {
    pointer_pos: Cell<Point<f64, Logical>>,
    pressed_keys: RefCell<HashSet<u32>>,
}

impl SeatState {
    pub fn for_seat(seat: &Seat<State>) -> &Self {
        seat.user_data().insert_if_missing(Self::default);
        seat.user_data().get::<Self>().unwrap()
    }

    pub fn pointer_pos(&self) -> Point<f64, Logical> {
        self.pointer_pos.get()
    }

    pub fn set_pointer_pos(&self, pointer_pos: Point<f64, Logical>) {
        self.pointer_pos.set(pointer_pos);
    }

    pub fn update_pressed_keys(&self, keysym: u32, state: KeyState) {
        if let KeyState::Pressed = state {
            self.pressed_keys.borrow_mut().insert(keysym);
        } else {
            self.pressed_keys.borrow_mut().remove(&keysym);
        }
    }
}
