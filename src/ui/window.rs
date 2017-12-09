extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use std::process::{ Command };
use gtk::{
    Window,
    MenuBar,
    MenuItem,
    Label,
    AboutDialog,
    Box,
    Orientation,
    ScrolledWindow,
    Widget,
    SelectionData,
    Button,
    TextView,
    TextBuffer
};
use std::mem::transmute;
use std::boxed::Box as Box_;
use gdk::Event;

pub fn show_window() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");

        return;
    }

    let mut window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Dictless");
    window.set_border_width(5);
   window.set_default_size(640, 480);
    window.set_visible(false);
    window.hide();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    let text = TextView::new();
 let button = TextView::new();
 button.get_buffer().unwrap().set_text("sdf\n dfsdf dsfsdf sf\nsd");
    let vbox = Box::new(Orientation::Vertical, 10);

    vbox.add(&create_menu(&window));
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    window.show_all();
    gtk::main();
}

fn dfsd(window : &Window, data : &SelectionData, num : u32) {
    println!("sdfs");
}

fn oll(window : &Window, event : &Event) -> Inhibit {
    println!("{:?}", event.get_event_type());
    return Inhibit(true)
}


fn create_menu(window: &Window) -> MenuBar {
    let menu_bar = MenuBar::new();
    let about = MenuItem::new_with_label("About");
    let window = window.to_owned();
    
    about.connect_activate(move |_| {
        let dialog = AboutDialog::new();
        dialog.set_authors(&["@akavolkol"]);
        dialog.set_title("About");
        dialog.set_copyright(Some("2017"));
        dialog.set_license(Some("MIT"));
        dialog.set_transient_for(Some(&window));
        
        dialog.run();
        dialog.destroy();
    });
    
    menu_bar.append(&about);

    menu_bar
}