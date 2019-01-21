#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change `forbid` to #![allow(unsafe_code)] to use `unsafe` keyword.
#![deny(warnings)]
#![forbid(overflowing_literals)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(missing_docs, clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented,
//        clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]

use structopt::StructOpt;

use minesweeper::{
    Args,
    Result,
    run,
};

fn main() -> Result<()> {
    run(Args::from_args())
}

