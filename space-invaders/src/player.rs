use crate::frame::{Drawable, Frame};
use crate::invaders::Army;
use crate::shot::Shot;
use crate::{NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};
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

    pub fn try_to_hit_something(&mut self, army: &mut Army) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding && army.try_to_kill_invader_at(shot.x, shot.y) {
                hit_something = true;
                shot.explode();
            }
        }

        hit_something
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
