use super::input;
use super::search;
use std::process;

pub fn run_cmd_query() {
    let cmd_input = input::get_cmd_input().unwrap_or_else(|err| {
        print!("Issue with receiving use arguments: {}\n", err);
        process::exit(1);
    });

    let is_in_file = search::search_for_query(&cmd_input).unwrap_or_else(|err| {
        print!("Issue with searching in file: {}\n", err);
        process::exit(1);
    });

    print!(
        "is query: {:#?} in file: {:#?}? answer: {:#?}\n",
        cmd_input.query, cmd_input.path, is_in_file
    );
}
