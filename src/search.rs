use super::input::CmdInput;
use std::error::Error;
use std::fs;

fn get_file_content(input: &CmdInput) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(&input.path[..])?)
}

pub fn search_for_query(input: &CmdInput) -> Result<bool, Box<dyn Error>> {
    let file_content = get_file_content(input);

    match file_content {
        Ok(content) => {
            return Ok(content.contains(&input.query[..]));
        }
        Err(err) => Err(err),
    }
}
