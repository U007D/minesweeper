use std::{
    char,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    io::BufRead,
    u16,
};

use crate::{
    consts::*,
    GameBoard,
};

#[cfg(test)]
mod unit_tests;
/// `View` is the 'V' in the MVC (Model-View-Controller) pattern.  This struct queries the model
/// (`game_board::GameBoard`) to determine state of each cell to display.  It also accepts user input and notifies
/// the model to update state based on the contents of the input message received from the user.
#[derive(Debug)]
pub struct View<T: BufRead> {
    model: GameBoard,
    reader: T,
    max_row_digits: usize,
    max_col_digits: usize,
}

impl<T: BufRead> View<T> {
    /// Constructor.
    pub fn new(model: GameBoard, reader: T) -> Self {
        // rows * cols should fit into a usize on a 32-bit machine (ie. a u32)
        debug_assert!(model.rows() <= u16::MAX as usize);
        debug_assert!(model.cols() <= u16::MAX as usize);
        // cannot truncate ∵ both `model.rows()` and `model.cols()` values are limited to 16 bits
        // cannot lose sign ∵ intermediate `f64` value is always positive
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Self {
            max_row_digits: (f64::from(model.rows() as u16).log10() + 1.0) as usize,
            max_col_digits: (f64::from(model.cols() as u16).log10() + 1.0) as usize,
            model,
            reader,
        }
    }

    fn col_headers(&self) -> String {
        const BASE_10: usize = 10;
        // cannot overflow ∵ `model.cols()` is bound to 16 bits which limits `max_row_digits` to 5 (3 bits).
        // TODO: fix possible overflow due to headers
        #[allow(clippy::integer_arithmetic)]
            let mut result = String::with_capacity((self.max_row_digits + 1 + self.model.cols()) * (self.max_col_digits + 1));
        (0..self.max_col_digits).rev()
                                .for_each(|exp| {
                                     result += &(" ".repeat(self.max_row_digits) + "|");
                                    // cannot truncate ∵ `exp` cannot exceed 5 (# of base 10 digits in u16::MAX).
                                    #[allow(clippy::cast_possible_truncation)]
                                        let pow = BASE_10.pow(exp as u32);
                                    // cannot overflow ∵ `model.cols()` is limited to 16 bits, which, in turn limits
                                    // `max_row_digits` to 5 base 10 digits (== 3 bits).
                                    // *1 cannot truncate ∵ `x % 10` never exceeds 9.
                                        #[allow(clippy::integer_arithmetic, clippy::cast_possible_truncation)]
                                    (1..=self.model.cols()).for_each(|c| {
                                        result += &match c < pow {
                                            true => ' ',
                                             // *1
                                            false => char::from_digit((c / pow % BASE_10) as u32, BASE_10 as u32)
                                                .expect(msg::ERR_INTERNAL_DIGIT_NOT_BASE_10),
                                        }.to_string();
                                     });
                                     result += "\n";
                                 });
        result += &("-".repeat(self.max_row_digits) + "+" + &"-".repeat(self.model.cols()) + "\n");
        result
    }

    fn row_header(&self, row: usize) -> String {
        format!("{1:>0$}|", self.max_row_digits, row.to_string())
    }
}

impl<T: BufRead> Display for View<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut body = String::new();
        (1..=self.model.rows()).for_each(|r| {
            body += &self.row_header(r);
            (1..=self.model.cols()).for_each(|_c| body += " ");
            body += "\n";
        });
        write!(f, "{}{}", self.col_headers(), body)
    }
}
