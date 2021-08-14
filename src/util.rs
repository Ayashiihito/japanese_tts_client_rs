use regex::Regex;

pub fn is_japanese(text: &str) -> bool {
  let is_japanese =
    Regex::new(r"[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]").unwrap();

  is_japanese.is_match(&text)
}
