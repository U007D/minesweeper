use std::io::BufRead;

use crate::GameBoard;

#[cfg(test)]
mod unit_tests;
/// `View` is the 'V' in the MVC (Model-View-Controller) pattern.  This struct queries the model
/// (`game_board::GameBoard`) to determine state of each cell to display.  It also accepts user input and notifies
/// the model to update state based on the contents of the input message received from the user.
#[derive(Debug)]
pub struct View<T: BufRead> {
    model: GameBoard,
    reader: T,
}

impl<T: BufRead> View<T> {
    /// Constructor.
    pub fn new(model: GameBoard, reader: T) -> Self {
        Self {
            model,
            reader,
        }
    }
}
