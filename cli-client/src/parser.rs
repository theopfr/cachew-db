use log::debug;


#[derive(Debug, PartialEq)]
pub enum ResponseStatus {
    OK,
    WARN,
    ERROR
}


#[derive(Debug, PartialEq)]
pub struct ParsedResponse {
    pub status: ResponseStatus,
    pub command: Option<String>,
    pub value: Option<String>,
}

/// Splits a string at a delimiter, unless the delimiter is in a substring enclosed by quotes.
/// 
/// # Arguments:
/// * `string`: The string to split.
/// * `delimiter`: The delimiter on which to split.
/// 
/// # Returns:
/// A vector containing the parts of the string (with quotes removed).
fn split_at_delimiter(string: &str, delimiter: char) -> Vec<&str> {
    let mut parts: Vec<&str> = Vec::new();
    let mut current_part = 0;
    let mut inside_quotes = false;
    let mut prev_char: Option<char> = None;

    for (index, ch) in string.char_indices() {
        match ch {
            '"' => {
                if prev_char != Some('\\') {
                    inside_quotes = !inside_quotes;
                }
            }
            ch if ch == delimiter && !inside_quotes => {
                let part = &string[current_part..index];
                parts.push(part.trim_matches(|c| c == ' '));
                current_part = index + ch.len_utf8();
            }
            _ => {}
        }
        prev_char = Some(ch);
    }

    let part = &string[current_part..];
    parts.push(part.trim_matches(|c| c == ' '));

    parts
}


/// Don't trust the server...
pub fn parse_response(response: &str) -> Result<ParsedResponse, String> {
    const CASP_PREFIX: &str = "CASP";
    const CASP_SUFFIX: &str = "\n";
    const OK_IDENTIFIER: &str = "OK";
    const WARN_IDENTIFIER: &str = "WARN";
    const ERROR_IDENTIFIER: &str = "ERROR";

    // split response parts at delimiter "/"
    

    // this doesnt work...
    let response_parts: Vec<&str> = split_at_delimiter(&response, '/');

    if response_parts.len() == 1 && response_parts[0] == "" {
        return Err("Failed to parse response: Received empty response.".to_string());
    }

    // check if the 'CASP' prefix exists
    if response_parts[0] != CASP_PREFIX {
        return Err("Failed to parse response: Prefix 'CASP' not found.".to_string());
    }

    // check if the '\n' suffix exists
    if response_parts.last().unwrap() != &CASP_SUFFIX {
        return Err(r#"Failed to parse response: Suffix '\n' not found."#.to_string());
    }

    // check if the response is OK, ERROR or something invalid
    match response_parts[1] {
        OK_IDENTIFIER => {
            if response_parts[2].starts_with("GET") {
                if response_parts.len() != 6 {
                    return Err(r#"Failed to parse response: Expected GET OK responses to consist of six parts (CASP + OK + <command> + <type> + <value> + \n)."#.to_string());
                }

                //let escaped_response = response_parts[4].replace("\"", "\\\"").to_string().replace("\\\"", "\"");


                Ok(ParsedResponse {
                    status: ResponseStatus::OK,
                    command: Some(response_parts[2].to_string()),
                    value: Some(response_parts[4].to_string()),
                })
            }
            else if response_parts[2].starts_with("EXISTS") || 
                    response_parts[2].starts_with("PING") || 
                    response_parts[2].starts_with("LEN") {
                if response_parts.len() != 5 {
                    return Err(r#"Failed to parse response: Expected EXISTS, PING, LEN OK responses to consist of five parts (CASP + OK + <command> + <message> + \n)."#.to_string());
                }

                Ok(ParsedResponse {
                    status: ResponseStatus::OK,
                    command: Some(response_parts[2].to_string()),
                    value: Some(response_parts[3].to_string()),
                })
            }
            else {
                if response_parts.len() != 4 {
                    return Err(r#"Failed to parse response: Expected OK responses to consist of four parts (CASP + OK + <command> + \n)."#.to_string());
                }
    
                Ok(ParsedResponse {
                    status: ResponseStatus::OK,
                    command: Some(response_parts[2].to_string()),
                    value: None,
                })
            }
        },
        WARN_IDENTIFIER => {
            if response_parts.len() != 4 {
                return Err(r#"Failed to parse response: Expected WARN responses to consist of four parts (CASP + WARN + <command> + \n)."#.to_string());
            }

            Ok(ParsedResponse {
                status: ResponseStatus::WARN,
                command: Some(response_parts[2].to_string()),
                value: None,
            })
        },
        ERROR_IDENTIFIER => {
            if response_parts.len() != 4 {
                return Err(r#"Failed to parse response: Expected ERROR responses to consist of four parts (CASP + ERROR + message + \n)."#.to_string());
            }

            Ok(ParsedResponse {
                status: ResponseStatus::ERROR,
                command: None,
                value: Some(response_parts[2].to_string()),
            })
        },
        _ => {
            Err("Failed to parse response: No status identifier found (expected one of: OK, ERROR).".to_string())
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_slash_delimiter() {

        let split_string: Vec<&str> = split_at_delimiter("CASP/OK/GET/STR/\"oh/no\"/\n", '/');
        assert_eq!(split_string, vec!["CASP", "OK", "GET", "STR", "\"oh/no\"", "\n"]);

        let split_string: Vec<&str> = split_at_delimiter("\"\"", '/');
        assert_eq!(split_string, vec!["\"\""]);

        let string = "CASP/OK/GET/INT/key \"value/1\"/\n";
        let split_string: Vec<&str> = split_at_delimiter(&string, '/');
        assert_eq!(split_string, vec!["CASP", "OK", "GET", "INT", "key \"value/1\"", "\n"]);

        let string = "\"A/B/C//D\"";
        let split_string: Vec<&str> = split_at_delimiter(&string, '/');
        assert_eq!(split_string, vec!["\"A/B/C//D\""]);

        let string = "CASP/OK/GET/STR/\"sdf\\\"sdf\"/\n";
        let split_string: Vec<&str> = split_at_delimiter(&string, '/');
        assert_eq!(split_string, vec!["CASP", "OK", "GET", "STR", "\"sdf\\\"sdf\"", "\n"]); 
    }

    #[test]
    fn test_parse_response() {
        // test success cases
        
        let parsed_response = parse_response("CASP/OK/SET/\n");
        assert_eq!(parsed_response, Ok(ParsedResponse { status: ResponseStatus::OK, command: Some("SET".to_string()), value: None }));

        let parsed_response = parse_response("CASP/OK/GET MANY/INT/10,20,30/\n");
        assert_eq!(parsed_response, Ok(ParsedResponse { 
            status: ResponseStatus::OK, command: Some("GET MANY".to_string()), value: Some("10,20,30".to_string())
        }));

        let parsed_response = parse_response("CASP/OK/LEN/10/\n");
        assert_eq!(parsed_response, Ok(ParsedResponse { status: ResponseStatus::OK, command: Some("LEN".to_string()), value: Some("10".to_string()) }));

        let parsed_response = parse_response("CASP/ERROR/An error appeared./\n");
        assert_eq!(parsed_response, Ok(ParsedResponse { status: ResponseStatus::ERROR, command: None, value: Some("An error appeared.".to_string()) }));

        let parsed_response = parse_response("CASP/WARN/SHUTDOWN/\n");
        assert_eq!(parsed_response, Ok(ParsedResponse { status: ResponseStatus::WARN, command: Some("SHUTDOWN".to_string()), value: None }));

        // test failures
        let parsed_response = parse_response("");
        assert_eq!(parsed_response.unwrap_err(), "Failed to parse response: Received empty response.");

        let parsed_response = parse_response("OK/SET/\n");
        assert_eq!(parsed_response.unwrap_err(), "Failed to parse response: Prefix 'CASP' not found.");

        let parsed_response = parse_response("CA/SP/OK/SET/\n");
        assert_eq!(parsed_response.unwrap_err(), "Failed to parse response: Prefix 'CASP' not found.");

        let parsed_response = parse_response("CASP/OK/GET MANY/1,2,3");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Suffix '\n' not found."#);

        let parsed_response = parse_response("CASP/SET/\n");
        assert_eq!(parsed_response.unwrap_err(), "Failed to parse response: No status identifier found (expected one of: OK, ERROR).");

        let parsed_response = parse_response("CASP/OK/SET/key/\n");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Expected OK responses to consist of four parts (CASP + OK + <command> + \n)."#);

        let parsed_response = parse_response("CASP/OK/GET/\"value/1\"/\n");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Expected GET OK responses to consist of six parts (CASP + OK + <command> + <type> + <value> + \n)."#);

        let parsed_response = parse_response("CASP/OK/EXISTS/\n");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Expected EXISTS, PING, LEN OK responses to consist of five parts (CASP + OK + <command> + <message> + \n)."#);

        let parsed_response = parse_response("CASP/ERROR/\n");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Expected ERROR responses to consist of four parts (CASP + ERROR + message + \n)."#);

        let parsed_response = parse_response("CASP/WARN/SHUTDOWN/Server shutting down!/\n");
        assert_eq!(parsed_response.unwrap_err(), r#"Failed to parse response: Expected WARN responses to consist of four parts (CASP + WARN + <command> + \n)."#);
    }
}