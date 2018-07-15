use std::process;
use std::fs::File;
use std::env;
use models::word::Word;
use serde_json;
use serde_json::Error;
use std::io::Read;
use utils::resources;

#[derive(Clone)]
pub struct Search {
    dictonary: Vec<Word>,
}

impl Search {
    pub fn new() -> Search {
        Search {
            dictonary: Vec::new(),
        }
    }

    fn get_dic_offline() -> Result<Vec<Word>, Error> {
        let mut path = resources::path();
        path.push("dictonaries");
        let mut file =
            File::open(format!("{}/uk.json", path.to_str().unwrap())).expect("file not found");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("could not read dictonary file");
        let dictionary: Vec<Word> = serde_json::from_str(&contents)?;
        Ok(dictionary)
    }

    pub fn search_words(&mut self, text: &String) -> Vec<Word> {
        if self.dictonary.len() == 0 {
            let dic = Search::get_dic_offline();
            let words = match dic {
                Ok(fonts_archive) => fonts_archive,
                Err(why) => {
                    eprintln!("failed to get font archive: {}", why);
                    process::exit(1);
                }
            };
            self.dictonary.extend(words.clone());
        }

        let mut found: Vec<Word> = Vec::new();
        for word in self.dictonary.iter() {
            if found.len() > 9 {
                break;
            }
            if word.original.starts_with(text) {
                found.push(word.clone())
            }
        }

        found
    }
}
