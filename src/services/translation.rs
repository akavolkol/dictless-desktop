use services::api;
use std::process;
use std::fs::File;
use std::env;
use models::dictonary::Dictonary;
use serde_json;
use serde_json::Error;
use std::io::Read;

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

    fn get_dic(self) -> Result<Vec<Dictonary>, Error> {
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.pop();
        path.pop();

        let mut f =
            File::open(format!("{}/data/uk.json", path.to_str().unwrap())).expect("file not found");
        let mut contents = String::new();
        println!("dsfs");
        f.read_to_string(&mut contents);
        let dictonary: Vec<Dictonary> = serde_json::from_str(&contents)?;

        Ok(dictonary)
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

    pub fn translate(self) -> String {
        let dic = self.get_dic();

        let text = match dic {
            Ok(fonts_archive) => fonts_archive[0].original.to_string(),
            Err(why) => {
                eprintln!("failed to get font archive: {}", why);
                process::exit(1);
            }
        };

        text.to_string()
    }
}
