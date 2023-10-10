use regex::Regex;

lazy_static! {
    static ref IS_JAPANESE_REGEX: Regex =
        Regex::new(r"[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]").unwrap();
}

pub fn is_japanese(text: &str) -> bool {
    IS_JAPANESE_REGEX.is_match(&text)
}
