extern crate futures;
extern crate gdk;
extern crate gtk;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
mod ui;
mod models;
mod services;

fn main() {
    init()
}

fn init() {
    ui::run();
}
