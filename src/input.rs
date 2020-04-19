use std::env;

#[derive(Debug)]
pub struct CmdInput {
    pub path: String,
    pub query: String,
}

pub trait NewCmdInput {
    fn new(args: &[String]) -> Result<CmdInput, &'static str>;
}

impl NewCmdInput for CmdInput {
    fn new(args: &[String]) -> Result<CmdInput, &'static str> {
        if args.len() < 3 {
            return Err("No enough input");
        }

        let path = args[1].clone();
        let query = args[2].clone();

        Ok(CmdInput { path, query })
    }
}

pub fn get_cmd_input() -> Result<CmdInput, &'static str> {
    let args: Vec<String> = env::args().collect();

    CmdInput::new(&args)
}

#[cfg(test)]
mod test {
    use super::*;

    fn cmd_input_mock(args: &[&str]) -> Result<CmdInput, &'static str> {
        let mock_cmd_raw_input = args
            .iter()
            .map(|&slice| String::from(slice))
            .collect::<Vec<String>>();

        CmdInput::new(&mock_cmd_raw_input)
    }

    #[test]
    fn cmd_input_success() {
        let args = &vec!["", "./text.txt", "hello"];

        let mock_cmd_input = cmd_input_mock(args);

        if let Err(err) = mock_cmd_input {
            panic!("Should return a Ok result instead of {:#?}", err);
        }
    }

    #[test]
    fn cmd_input_fail() {
        let args = &vec![""];

        let mock_cmd_input = cmd_input_mock(args);

        if let Ok(res) = mock_cmd_input {
            panic!("Should return a Err result instead of {:#?}", res);
        }
    }
}
