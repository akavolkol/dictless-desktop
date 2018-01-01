use gtk::{SearchEntry, Application, ListBox, ListBoxRow, Label};
use gtk::prelude::*;
use models::word::Word;
use services::translation::Translator;

#[derive(Clone)]
pub struct Sidebar {
    pub search_bar: SearchEntry,
    pub items_menu: ListBox,
    words: Vec<Word>
}

impl Sidebar {
    pub fn new() -> Sidebar {
        let search_bar = SearchEntry::new();
        let items_menu = ListBox::new();

        let fonts_archive = [
            Word {
                original: String::from("book"),
                translation: String::from("книга"),
            },
        ];
        for font in fonts_archive.iter() {
            let row = FontRow::new(font.original.clone(), font.translation.clone());
            items_menu.insert(&row.container, -1);
        }

        Sidebar {
            search_bar,
            items_menu,
            words: Vec::new()
        }
    }

    pub fn set_words(mut self, words: Vec<Word>) {
        self.words = words;
    }

    pub fn on_search<F: Fn(&String) + 'static>(&mut self, callback: F) {
        let inst = self.clone();
        println!("{}", 1);
        self.search_bar.connect_search_changed(move |search| {
            println!("{}", 2);
            let s = inst.clone();
            let buffer = search.get_buffer();
            let text = buffer.get_text();
            s.set_words(vec![Word {
                original: String::from("book"),
                translation: String::from("книга"),
            }]);
            callback(&text);
        });
    }

    pub fn on_word_selection(&mut self) {
        let inst = self.clone();
        println!("{}", 3);
        self.items_menu.connect_row_selected(move |_, row| {
            println!("{}", inst.words.len());
            if let Some(row) = row.as_ref() {
                let mut translator = Translator::new("en".to_string(), "uk".to_string());
                let id = row.get_index() as usize;

                if (inst.words.len() >= id) {
                    let word = &inst.words[0].clone();
                    println!("{}", translator.translate(&word.original));
                }
            }
        });
    }
}

#[derive(Clone)]
pub struct FontRow {
    pub container: ListBoxRow,
    pub category: String,
    pub family: String,
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

        FontRow {
            container,
            category,
            family,
        }
    }

    pub fn set_visibility(&self, visibility: bool) {
        self.container.set_visible(visibility);
    }

    pub fn contains(&self, pattern: &str) -> bool {
        // TODO: do this without making any allocations.
        self.family.to_lowercase().contains(&pattern.to_lowercase())
    }
}