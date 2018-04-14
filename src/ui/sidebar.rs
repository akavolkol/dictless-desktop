use gtk::{
    SearchEntry,
    Align,
    ListBox,
    ListBoxRow,
    Label,
    Builder
};
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
        items_menu.set_border_width(5);

        Sidebar {
            search_bar,
            items_menu,
            words: Rc::new(RefCell::new(vec![]))
        }
    }

    pub fn set_words(mut self, words: Rc<RefCell<Vec<Word>>>) {
        self.words = words;
        for row in self.items_menu.get_children().iter() {
            self.items_menu.remove(row);
        }
        for word in self.words.borrow().iter() {
            let row = WordRow::new(word.original.clone(), word.translation[0].value.clone());
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
                let mut words: Vec<Word> = vec!();
                match api::get_words(&text) {
                    Ok(data) => {
                        if !data.is_none() {
                            words = data.unwrap();
                        }
                    },
                    Err(why) => {
                        eprintln!("failed to get words: {}", why);
                        process::exit(1);
                    }
                };

                // TODO: figure out how to deal with borrowing
                inst1.words.borrow_mut().clear();
                inst1.words.borrow_mut().append(&mut words.clone());
                inst1.set_words( Rc::new(RefCell::new(words.clone())));

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
                    let mut text = String::from("");
                    let mut translations = vec![];
                    for translation in word.translation.iter() {
                        translations.push(translation.value.as_str());
                    }
                    text = translations.join(", ");
                    callback(&text);
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
        let builder = Builder::new_from_resource("/com/dictless/ui/words_list/row.xml");
        let container: ListBoxRow = builder.get_object("row").unwrap();

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
