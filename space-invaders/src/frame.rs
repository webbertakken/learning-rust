use crate::{NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut columns = Vec::with_capacity(NUMBER_OF_COLUMNS);
    for _ in 0..NUMBER_OF_COLUMNS {
        let mut column = Vec::with_capacity(NUMBER_OF_ROWS);
        for _ in 0..NUMBER_OF_ROWS {
            column.push(" ");
        }

        columns.push(column);
    }

    columns
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
