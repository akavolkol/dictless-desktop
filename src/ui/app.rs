extern crate gtk;

use ui::sidebar::Sidebar;
use ui::window;

use gtk::prelude::*;
use gtk::{AboutDialog, Box, Label, ListBox, ListBoxRow, MenuBar, MenuItem, Orientation,
          ScrolledWindow, SearchEntry, Separator, TextBuffer, TextView, Window};

pub struct App {
    pub window: Window,
    pub sidebar: Sidebar,
}

pub fn new() -> App {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
    }

    let window = window::new();
    let scroller = ScrolledWindow::new(None, None);
    scroller.set_min_content_width(200);
    let mut sidebar = Sidebar::new();

    let content = Box::new(Orientation::Vertical, 5);
    let content_text = TextBuffer::new(None);
    let content_view = TextView::new_with_buffer(&content_text);
    content_view.set_editable(false);

    let label = Label::new("Translation");

    sidebar.on_search(move |text: &String| {
        let k = TextBuffer::new(None);
        k.set_text(text);
        content_text.set_text(text);
    });

    sidebar.on_word_selection();

    scroller.add(&content_view);
    content.pack_start(&label, false, false, 0);
    content.pack_start(&scroller, true, true, 0);

    let view = Box::new(Orientation::Horizontal, 0);
    let sidebarView = Box::new(Orientation::Vertical, 0);
    sidebarView.pack_start(&sidebar.search_bar, false, false, 0);
    sidebarView.pack_start(&sidebar.items_menu, false, false, 0);
    view.pack_start(&sidebarView, false, false, 0);
    view.pack_start(&Separator::new(Orientation::Vertical), false, false, 0);
    view.pack_start(&scroller, true, true, 0);
    view.pack_start(&content, true, true, 0);

    window.add(&view);
    window.show_all();
    gtk::main();

    App {
        window,
        sidebar: sidebar,
    }
}