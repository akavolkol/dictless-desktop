extern crate gtk;

use gtk::prelude::*;
use gtk::{Window};

pub fn new() -> Window {
    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Dictless");
    window.set_border_width(5);
    window.set_default_size(640, 480);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window
}


