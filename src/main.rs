extern crate futures;
extern crate gdk;
extern crate gtk;
extern crate hyper;
extern crate tokio_core;

mod ui;
mod models;
mod services;

fn main() {
    init()
}

fn init() {
    ui::run();
}
