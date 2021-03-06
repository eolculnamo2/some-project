pub struct TranslationFnDetails {
  pub fn_text: String,
  pub file_index: u16,
}

pub struct DomMetaData {
  pub path: String,
  pub translation_details: Vec<TranslationFnDetails>,  
}
