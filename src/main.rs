
extern crate gtk;
extern crate glib;

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
    SelectionData
};
use glib::object::GObject;
use glib::signal::connect;
use std::mem::transmute;
use std::boxed::Box as Box_;

struct Memory {
    total: i32,
    swap: i32
}

fn main() {
    init();
}

fn init() {
    
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");

        return;
    }

    /*

    void Selection::create_selection_widget()
{
	if (selection_widget)
		gtk_widget_destroy(selection_widget);
	selection_widget = gtk_invisible_new ();
	g_signal_connect (G_OBJECT (selection_widget), "selection_received", G_CALLBACK (SelectionReceivedCallback), this);
}

    */

    let mut window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hardware info");
    window.set_border_width(5);
    window.set_default_size(640, 480);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let vbox = Box::new(Orientation::Vertical, 10);
    // let scrolled_window = ScrolledWindow::new(None, None);
    // scrolled_window.add(&render_memory());
    // let mut wid = Widget {
    //     g_type_instance:,
    //     qdata,
    //     ref_count
    // };
    window.connect_selection_received(dfsd);
    println!("Failed to initialize GTK.");
    //let mut wid = WidgetExt::new();
    //connect(&wid, "activate", transmute(dfsd as usize), Box_::new("dsf") as *mut _);

    vbox.add(&create_menu(&window));
    //vbox.pack_start(&scrolled_window, true, true, 0);
    window.add(&vbox);

    window.show_all();
    gtk::main();
}

fn dfsd(window : &Window, data : &SelectionData, num : u32) {
    println!("sdfs");
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

fn mem_info() -> Memory {
    let output = Command::new("cat")
    .arg("/proc/meminfo")
    .output()
    .unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });


    let mut memory = Memory { total: 0, swap: 0 };
   
    memory
}
