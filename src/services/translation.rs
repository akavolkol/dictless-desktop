use services::api;
use std::process;
use models::dictonary::Dictonary;

pub struct Translator {
    source_language: String,
    destination_language: String,
    dictonary: Option<Dictonary>,
}

impl Translator {
    pub fn new(source_language: String, destination_language: String) -> Translator {
        Translator {
            source_language,
            destination_language,
            dictonary: None,
        }
    }

    pub fn translate_request(self, text: &String) -> String {
        let mut text = String::new();
        text = match api::translate(&text) {
            Ok(fonts_archive) => fonts_archive.text,
            Err(why) => {
                eprintln!("failed to get font archive: {}", why);
                process::exit(1);
            }
        };
        text.to_string()
    }
}
