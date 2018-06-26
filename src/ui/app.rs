extern crate gtk;

use ui::sidebar::Sidebar;
use ui::window;
use gio;
use gio::ApplicationFlags;
use gio::{resources_register, Resource};
use std::error::Error;
use std::env;
use gtk::prelude::*;
use gtk::{AboutDialog, Box, Builder, Label, MenuBar, MenuItem, Orientation, ScrolledWindow,
          Separator, Stack, TextBuffer, TextView, Window};
use gdk::Screen;

pub struct App {
    pub window: Window,
}

pub fn new() -> App {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
    }
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.pop();
    match Resource::load(format!(
        "{}/resources/bundle.gresource",
        path.to_str().unwrap()
    )) {
        Ok(resource) => resources_register(&resource),
        Err(error) => println!("{:?}", gio::Error::description(&error)),
    }

    gtk::Application::new("com.dictless", ApplicationFlags::empty())
        .expect("Initialization failed.");

    let screen = Screen::get_default().unwrap();
    let provider = gtk::CssProvider::new();

    let css_path = format!("{}/resources/ui/styles.css", path.to_str().unwrap());
    provider.load_from_path(&*css_path).unwrap();

    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let window = window::new();
    let scroller = ScrolledWindow::new(None, None);
    scroller.set_min_content_width(200);
    let mut sidebar = Sidebar::new();

    let builder = gtk::Builder::new_from_resource("/com/dictless/ui/content.xml");
    let content: Box = builder.get_object("content").unwrap();

    let content_empty: Box = builder.get_object("content-empty").unwrap();
    let stack = Stack::new();
    let content_text = TextBuffer::new(None);
    let content_view: TextView = builder.get_object("translation").unwrap();
    content_view.set_buffer(&content_text);
    content_view.set_editable(false);

    stack.add_named(&content_empty, "empty");
    stack.add_named(&content_view, "text");
    stack.set_visible_child_name("empty");
    scroller.add(&stack);

    let label = Label::new("Translation");
    sidebar.on_word_selection(move |text: &String| {
        content_text.set_text(text);
        &stack.set_visible_child_name("text");
    });
    sidebar.setup_search();
    content.pack_start(&label, false, false, 0);
    content.pack_start(&scroller, true, true, 5);

    let builder = Builder::new_from_resource("/com/dictless/ui/view.xml");
    let app_view = Box::new(Orientation::Vertical, 0);
    let view: Box = builder.get_object("view").unwrap();
    let sidebar_view = Box::new(Orientation::Vertical, 0);
    sidebar_view.pack_start(&sidebar.search_bar, false, false, 0);
    sidebar_view.pack_start(&sidebar.items_menu, false, false, 5);

    let menu_bar = MenuBar::new();
    let about = MenuItem::new_with_label("Про програму");
    let window_moved = window.to_owned();
    about.connect_activate(move |_| {
        let dialog = AboutDialog::new();
        dialog.set_title("Про програму");
        dialog.set_copyright(Some(
            "
        Libraries used:
        GTK
        ",
        ));
        dialog.set_license(Some("MIT"));
        dialog.set_transient_for(Some(&window_moved));
        dialog.run();
        dialog.destroy();
    });
    menu_bar.append(&about);

    view.pack_start(&sidebar_view, false, false, 0);
    view.pack_start(&Separator::new(Orientation::Vertical), false, false, 0);
    view.pack_start(&content, true, true, 0);
    app_view.add(&menu_bar);
    app_view.add(&view);
    window.add(&app_view);
    window.show_all();
    gtk::main();

    App { window }
}
