extern crate gtk;

use gtk::prelude::*;
pub mod window;
pub mod sidebar;
mod app;

pub fn run() {
    app::new();
}
