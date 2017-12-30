extern crate gtk;

use gtk::prelude::*;
use gtk::{
    Window,
    MenuBar,
    MenuItem,
    Label,
    AboutDialog,
    Box,
    Orientation,
    ScrolledWindow,
    TextView,
    ListBox,
    ListBoxRow,
    SearchEntry,
    Separator
};

pub struct Font {
    pub kind:     String,
    pub family:   String,
    pub category: String,
}

pub fn show_window() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");

        return;
    }

    let mut window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Dictless");
    window.set_border_width(5);

    window.set_default_size(640, 480);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let fonts_box = ListBox::new();
    let fonts_archive = [
        Font {
            kind: String::from("sdfs"),
            category: String::from("cat"),
            family: String::from("famile")
        },
        Font {
            kind: String::from("sdfs"),
            category: String::from("cat"),
            family: String::from("famile")
        },
        Font {
            kind: String::from("sdfs"),
            category: String::from("cat"),
            family: String::from("famile")
        }
    ];
    for font in fonts_archive.iter() {
        let row = FontRow::new(
            font.category.clone(),
            font.family.clone(),
        );
        fonts_box.insert(&row.container, -1);
    }

    let scroller = ScrolledWindow::new(None, None);
    scroller.set_min_content_width(200);
    scroller.add(&fonts_box);
    
    let search = SearchEntry::new();

    let text = TextView::new();
    let button = TextView::new();
    button.get_buffer().unwrap().set_text("sdf\n dfsdf dsfsdf sf\nsd");

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.pack_start(&create_menu(&window), false, false, 0);
    vbox.pack_start(&search, false, false, 0);
    vbox.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
    vbox.pack_start(&scroller, true, true, 0);
    vbox.pack_start(&button, false, false, 0);

    window.add(&vbox);

    window.show_all();
    gtk::main();
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

#[derive(Clone)]
pub struct FontRow {
    pub container: ListBoxRow,
    pub category:  String,
    pub family:    String,
}

impl FontRow {
    pub fn new(category: String, family: String) -> FontRow {
        // Create the inner label of the row that contains the family in bold.
        let label = Label::new("");
        label.set_markup(&["<b>", family.as_str(), "</b>"].concat());
        //label.set_halign(Align::Start);
        label.set_margin_top(5);

        // Store the label within the list box row.
        let container = ListBoxRow::new();
        container.add(&label);

        FontRow { container, category, family }
    }

    pub fn set_visibility(&self, visibility: bool) { self.container.set_visible(visibility); }

    pub fn contains(&self, pattern: &str) -> bool {
        // TODO: do this without making any allocations.
        self.family.to_lowercase().contains(&pattern.to_lowercase())
    }
}
