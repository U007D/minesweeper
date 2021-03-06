#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change `forbid` to #![allow(unsafe_code)] to use `unsafe` keyword.
//#![deny(warnings, missing_docs)]
#![forbid(overflowing_literals)]
// Uncomment before ship to ensure complete docs, reconcile use of possibly redundant crates and uncover debugremnants
//#![warn(missing_docs, clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented,
//        clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]
//! # Minesweeper
//!
//! This is a practice command-line implementation of the famous
//! [Minesweeper](https://en.wikipedia.org/wiki/Minesweeper_(video_game)) game.
//!
//! ## Invoking the game
//! To invoke it from the command line, use `cargo run <rows> <columns>`, where `<rows>` and `<columns>` are the  number
//! of rows and columns you want the game board to have.
//!
//! ## Game Play
//! On each turn, enter the row & column number of the board cell to reveal.  If you reveal all non-mined cells, on
//! the game board, you win.  If you reveal a mine, you lose.
//! Please see [Minesweeper](https://en.wikipedia.org/wiki/Minesweeper_(video_game)) for detailed instructions.
//!
//! ## Design
//! Minesweeper uses a basic Model-View-Controller (MVC) pattern.  The locations of mines are stored in a structure
//! representing the board as the Model (`game_board::GameBoard`).  The state of revealed cells (revealed vs not
//! revealed, as well as the number of adjacent mines) on the game board and user input on what to reveal next is
//! managed by `view::View`.  And finally, the rules of gameplay are enforced by the Controller (`minesweeper::run()`).
use std::{
    io::stdin,
    result::Result as StdResult
};

pub use {
    args::Args,
    consts::*,
    error::Error,
    game_board::GameBoard,
    ranged_num::{
        BoardDimension,
        Probability,
    },
    view::View,
};

mod args;
mod consts;
mod error;
mod game_board;
mod ranged_num;
mod view;
/// Convenience alias for the `Result` type encoding `minecraft::error::Error` as the default error type.
pub type Result<T> = StdResult<T, Error>;

/// Library entry point.
pub fn run(args: Args) -> Result<()> {
    println!("{:?}", args);
    let model = GameBoard::new(args.rows, args.cols, args.prob);
    let stdin = stdin();
    let reader = stdin.lock();
    let view = View::new(model, reader);
    println!("{}", view);
    Ok(())
}
