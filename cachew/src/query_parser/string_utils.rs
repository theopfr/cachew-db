use regex::Regex;


/// Splits a string at its spaces, unless enclosed by quotes.
/// 
/// # Arguments:
/// * `string`: The string to split.
/// 
/// # Returns:
/// A vector containing the parts of the string (with quotes removed).
pub fn split_whitespaces(string: &str) -> Vec<&str>{
    let regex = Regex::new(r#""[^"]+"|\S+"#).unwrap();
    regex.find_iter(string).map(|m| {
        let matched_string: &str = m.as_str();
        if matched_string.contains(' ') {
            return matched_string.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        }
        matched_string
    }).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_whitespaces() {
        let split_string: Vec<&str> = split_whitespaces("test test \"in quotes\" test \"in quotes\"");
        assert_eq!(split_string, vec!["test", "test", "in quotes", "test", "in quotes"]);
    }
}