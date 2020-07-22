use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    // indexing by x for
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    // an imutable reference to yourself, a mutable reference to the frame
    // that's the logic
    fn draw(&self, frame: &mut Frame);
}
