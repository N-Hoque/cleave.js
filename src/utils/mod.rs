pub struct Util;

impl Util {
    pub fn noop() {}

    pub fn strip(value: &str, replacement: &str) -> String {
        value.replace(replacement, "")
    }

    pub fn get_post_delimiter(value: &str, delimiter: &str, delimiters: &[&str]) -> &str {
        if delimiters.is_empty() {
            return if value[(value.len() - delimiter.len())..] == delimiter { delimiter } else { "" };
        }

        let mut matched_delimiter = "";

        for delim in delimiters {
            if value[(value.len() - delim.len())..] == delim {
                matched_delimiter = delim;
            }
        }

        matched_delimiter
    }


}