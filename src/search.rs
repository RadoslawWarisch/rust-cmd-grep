use super::input::CmdInput;
use std::error::Error;
use std::fs;

fn get_file_content(input: &CmdInput) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(&input.path[..])?)
}

fn is_query_in_content(content: String, input: &CmdInput) -> bool {
    let query = &input.query[..];

    if input.is_case_sensitive {
        return content.contains(query);
    } else {
        return content.to_lowercase().contains(&query.to_lowercase()[..]);
    }
}

pub fn search_for_query(input: &CmdInput) -> Result<bool, Box<dyn Error>> {
    let file_content = get_file_content(input);

    match file_content {
        Ok(content) => {
            return Ok(is_query_in_content(content, input));
        }
        Err(err) => Err(err),
    }
}
