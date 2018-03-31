extern crate gtk;

pub mod window;
pub mod sidebar;
mod app;

pub fn run() {
    app::new();
}
