//! Remove the substring from named
//! content string.
//!
use kmpsearch::Haystack;

pub fn replace(content: String, substr: String) -> String {
    let content_slice: &str = &content;
    let substr_slice: &str = &substr;
    if let Some(indexes) = content_slice.indexesof_needle(substr_slice) {
        let mut constructed = String::new();
        let start_index: usize = indexes[0];
        let end_index: usize = indexes[0] + substr.len();
        let total_length: usize = content.len();
        let piece_one: &str = content.get(0..start_index).unwrap();
        let piece_two: &str = content.get(end_index..total_length).unwrap();
        constructed.push_str(piece_one);
        constructed.push_str(piece_two);
        constructed
    } else {
        content
    }
}

#[cfg(test)]
mod tests {

    use super::replace;
    #[test]
    fn test_replace_case_one() {
        let s1 = String::from("SHARKONDART");
        let s2 = String::from("ON");
        let new_string = replace(s1, s2);
        assert_eq!(new_string, "SHARKDART");
    }

    #[test]
    fn test_replace_case_two() {
        let s1 = String::from("TYSON");
        let s2 = String::from("SON");
        let new_string = replace(s1, s2);
        assert_eq!(new_string, "TY");
    }

    #[test]
    fn test_replace_case_with_spaces() {
        let s1 = String::from("TYS ON");
        let s2 = String::from("ON");
        let new_string = replace(s1, s2);
        assert_eq!(new_string, "TYS ");
    }

    #[test]
    fn test_replace_case_with_spaces_substr() {
        let s1 = String::from("TYS ON");
        let s2 = String::from(" ON");
        let new_string = replace(s1, s2);
        assert_eq!(new_string, "TYS");
    }
}
