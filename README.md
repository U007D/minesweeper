Command-line [Minesweeper](https://en.wikipedia.org/wiki/Minesweeper_(video_game)) game implemented in Rust as an
exercise.  (Currently under development)

Although this code is a simple console implementation of Minesweeper, this code was written as if it were a 
business-critical application adhering to the following **prioritized** tenets:
1) correctness (performs correctly and reliably (i.e. no panics))
2) maintainability 
(principlaly loose coupling via SOLID, use of typestate, and expressive functional style wherever possible, 
documentation) and
3) performance

In particular, note the crate-wide strict warning settings and the safe consideration/handling/elision of indexing, 
bounds-checking, arithmetic overflow, panics, floating-point arithmetic and floating-point precision with respect to 
potentially lossy casts. Inversion of control and semantic logging have been omitted for brevity.

Additionally, this code was developed using a form of of Test Driven Development to evolve the solution through a series
of [red green refactor](https://blog.cleancoder.com/uncle-bob/2014/12/17/TheCyclesOfTDD.html) cycles.
 
## Usage
#### [Install Rust](https://www.rust-lang.org/tools/install) (if not already installed)
* Linux, Unix or Mac OS X: `curl https://sh.rustup.rs -sSf | sh`.
* Windows: Download and run
[rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe).
#### Get this repository
`git clone https::github.com/u007d/minesweeper && cd minesweeper`
#### Run tests
`cargo test`
#### Build (& open) documentation
`cargo doc --open`
#### Run the game
`cargo run --release`
