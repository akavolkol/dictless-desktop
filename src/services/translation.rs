use services::api;
use std::process;

pub struct Translator {
    source_language: String,
    destination_language: String,
}

impl Translator {
    pub fn new(
        source_language: String,
        destination_language: String,
    ) -> Translator {
        Translator {
            source_language,
            destination_language,
        }
    }

    pub fn translate(self, text: &String) -> String {
        let text = match api::translate(text) {
            Ok(fonts_archive) => {
                fonts_archive.text
            },
            Err(why) => {
                eprintln!("failed to get font archive: {}", why);
                process::exit(1);
            }
        };
        text.to_string()
    }
}
