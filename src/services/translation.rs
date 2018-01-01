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
        "df".to_string()
    }
}
