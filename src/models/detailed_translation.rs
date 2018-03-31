#[derive(Clone)]
#[derive(Deserialize)]
pub struct DetailedTranslation {
    pub context:  Option<String>,
    pub value:  String,
    pub weight:  u32
}