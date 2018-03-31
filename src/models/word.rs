use models::detailed_translation::DetailedTranslation;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct Word {
    pub explanation:  String,
    pub original:  String,
    pub translation:  Vec<DetailedTranslation>
}