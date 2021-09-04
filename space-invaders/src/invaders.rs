use crate::frame::{Drawable, Frame};
use crate::{NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};
use rusty_time::timer::Timer;
use std::cmp::max;
use std::time::Duration;

pub enum Direction {
    Left,
    Right,
}
pub struct Invader {
    pub x: usize,
    pub y: usize,
}

impl Invader {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Army {
    pub invaders: Vec<Invader>,
    move_timer: Timer,
    direction: Direction,
}

impl Army {
    pub fn new() -> Self {
        let mut invaders = Vec::new();

        for x in 0..NUMBER_OF_COLUMNS {
            for y in 0..NUMBER_OF_ROWS {
                // don't hit the sides and use even squares only
                if x > 1 && x < NUMBER_OF_COLUMNS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0
                {
                    invaders.push(Invader::new(x, y));
                }
            }
        }

        Self {
            invaders,
            move_timer: Timer::from_millis(2000),
            direction: Direction::Right,
        }
    }

    pub fn try_to_move(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();

            let mut downwards = false;
            if matches!(self.direction, Direction::Left) {
                let minimum_x = self
                    .invaders
                    .iter()
                    .map(|invader| invader.x)
                    .min()
                    .unwrap_or(0);
                if minimum_x <= 0 {
                    self.direction = Direction::Right;
                    downwards = true;
                }
            } else {
                let maximum_x = self
                    .invaders
                    .iter()
                    .map(|invader| invader.x)
                    .max()
                    .unwrap_or(NUMBER_OF_COLUMNS - 1);

                if maximum_x >= NUMBER_OF_COLUMNS - 1 {
                    self.direction = Direction::Left;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.invaders.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.invaders.iter_mut() {
                    match self.direction {
                        Direction::Left => invader.x -= 1,
                        Direction::Right => invader.x += 1,
                    }
                }
            }

            return true;
        }

        false
    }

    pub fn is_destroyed(&self) -> bool {
        self.invaders.is_empty()
    }

    pub fn has_reached_the_bottom(&self) -> bool {
        self.invaders
            .iter()
            .any(|invader| invader.y >= NUMBER_OF_ROWS - 1)
    }

    pub fn try_to_kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(index) = self
            .invaders
            .iter_mut()
            .position(|invader| invader.x == x && invader.y == y)
        {
            self.invaders.remove(index);
            return true;
        }

        false
    }
}

impl Drawable for Army {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.invaders.iter() {
            let cycle_progress =
                self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();

            frame[invader.x][invader.y] = if cycle_progress >= 0.5 { "x" } else { "X" }
        }
    }
}
