use models::detailed_translation::DetailedTranslation;

#[derive(Clone, Deserialize)]
pub struct Dictonary {
	pub original: String,
	pub translation: Vec<DetailedTranslation>,
	pub explanation: Option<String>,
}
