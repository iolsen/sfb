#[macro_use]
extern crate approx;

mod hex;
mod screen;
mod ship;
mod ship_spec;

fn main() {
    screen::run().unwrap();
}
