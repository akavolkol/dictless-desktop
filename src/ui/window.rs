extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, Builder};

pub fn new() -> Window {
    // let builder = Builder::new_from_resource("/com/dictless/ui/window.xml");
    // let window: Window = builder.get_object("app_window").unwrap();
    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Dictless");
    window.set_default_size(640, 480);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window
}


