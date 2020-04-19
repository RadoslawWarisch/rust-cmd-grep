use std::env;

#[derive(Debug)]
pub struct CmdInput {
    pub path: String,
    pub query: String,
    pub is_case_sensitive: bool,
}

pub trait NewCmdInput {
    fn new(
        args: &[String],
        env_var: &Result<String, env::VarError>,
    ) -> Result<CmdInput, &'static str>;
}

impl NewCmdInput for CmdInput {
    fn new(
        args: &[String],
        env_var: &Result<String, env::VarError>,
    ) -> Result<CmdInput, &'static str> {
        if args.len() < 3 {
            return Err("No enough input");
        }

        let path = args[1].clone();
        let query = args[2].clone();
        let is_case_sensitive = !env_var.is_err();

        Ok(CmdInput {
            path,
            query,
            is_case_sensitive,
        })
    }
}

pub fn get_cmd_input() -> Result<CmdInput, &'static str> {
    let args: Vec<String> = env::args().collect();
    let env_var = env::var("CASE_SENSITIVE");

    CmdInput::new(&args, &env_var)
}

#[cfg(test)]
mod test {
    use super::*;

    fn cmd_input_mock(args: &[&str]) -> Result<CmdInput, &'static str> {
        let mock_args = args
            .iter()
            .map(|&slice| String::from(slice))
            .collect::<Vec<String>>();

        let mock_env_var = Ok(String::from("its okay"));

        CmdInput::new(&mock_args, &mock_env_var)
    }

    #[test]
    fn cmd_input_success() {
        let args = &vec!["", "./text.txt", "hello"];

        assert!(cmd_input_mock(args).is_ok());
    }

    #[test]
    fn cmd_input_fail() {
        let args = &vec![""];

        assert!(cmd_input_mock(args).is_err());
    }
}
