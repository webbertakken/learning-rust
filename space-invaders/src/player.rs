use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
use crate::{NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};
use std::ops::Deref;
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUMBER_OF_COLUMNS / 2,
            y: NUMBER_OF_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUMBER_OF_COLUMNS - 1 {
            self.x += 1;
        }
    }

    pub fn try_to_shoot(&mut self) -> bool {
        if self.shots.len() >= 3 {
            return false;
        }

        self.shots.push(Shot::new(self.x, self.y - 1));
        true
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead())
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
