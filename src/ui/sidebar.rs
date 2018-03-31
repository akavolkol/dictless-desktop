use gtk::{SearchEntry, Align, ListBox, ListBoxRow, Label};
use gtk::prelude::*;
use models::word::Word;
// use services::translation::Translator;
use services::api;
use std::process;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sidebar {
    pub search_bar: SearchEntry,
    pub items_menu: ListBox,
    words:  Rc<RefCell<Vec<Word>>>
}

impl Sidebar {
    pub fn new() -> Sidebar {
        let search_bar = SearchEntry::new();
        let items_menu = ListBox::new();

        Sidebar {
            search_bar,
            items_menu,
            words: Rc::new(RefCell::new(vec![]))
        }
    }

    pub fn set_words(mut self, words: Rc<RefCell<Vec<Word>>>) {
        self.words = words;
        for font in self.words.borrow().iter() {
            let row = WordRow::new(font.original.clone(), font.translation[0].value.clone());
            self.items_menu.insert(&row.container, -1);
        }
        self.items_menu.show_all();
    }

    pub fn setup_search(&mut self) {
        let inst = self.clone();
        self.search_bar.connect_search_changed(move |search| {
            let inst1 = inst.clone();
            let buffer = search.get_buffer();
            let text = buffer.get_text();
            if text != "" {
                let words: Vec<Word>;
                match api::get_words(&text) {
                    Ok(data) => {
                        words = data;
                    },
                    Err(why) => {
                        eprintln!("failed to get words: {}", why);
                        process::exit(1);
                    }
                };
                inst.words.borrow_mut().append(&mut words.clone());
                inst1.set_words( Rc::new(RefCell::new(words)));
            }
        });
    }

    pub fn on_word_selection<F: Fn(&String) + 'static>(&mut self, callback: F) {
        let inst = self.clone();
        self.items_menu.connect_row_selected(move |_, row| {
            if let Some(row) = row.as_ref() {
                // let translator = Translator::new("en".to_string(), "uk".to_string());
                let id = row.get_index() as usize;

                if inst.words.borrow().len() > id {
                    let word = &inst.words.borrow().clone()[id].clone();
                    callback(&word.translation[0].value);
                }
            }
        });
    }
}

#[derive(Clone)]
pub struct WordRow {
    pub container: ListBoxRow,
    pub original: String,
    pub translation: String,
}

impl WordRow {
    pub fn new(original: String, translation: String) -> WordRow {
        let label = Label::new("");
        label.set_markup(&["<b>", original.as_str(), "</b>"].concat());
        label.set_halign(Align::Start);
        label.set_margin_top(5);

        let container = ListBoxRow::new();
        container.add(&label);

        WordRow {
            container,
            original,
            translation,
        }
    }

    pub fn set_visibility(&self, visibility: bool) {
        self.container.set_visible(visibility);
    }

    pub fn contains(&self, pattern: &str) -> bool {
        // TODO: do this without making any allocations.
        self.original.to_lowercase().contains(&pattern.to_lowercase())
    }
}
