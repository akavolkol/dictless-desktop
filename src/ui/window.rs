extern crate gtk;

use gtk::prelude::*;
use gtk::{Builder, Window};

pub fn new() -> Window {
    let builder = Builder::new_from_resource("/com/dictless/ui/window.xml");
    let window: Window = builder.get_object("app_window").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window
}
