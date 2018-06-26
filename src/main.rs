extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod ui;
mod models;
mod services;

fn main() {
    ui::run();
}
