pub mod cmd_assemble {
    use std::process::Command;

    pub fn assemble(cmd: String, args: Vec<String>) -> Command {
        let mut ret: Command = Command::new(cmd);
        ret.args(args);
    
        ret
    }

    pub fn parse(args: Vec<String>) -> (String, Vec<String>) {
        let cmd: String = args[1].to_string();
        let arguments: Vec<String> = args[2..].to_vec();

        (cmd, arguments)
    }

    pub fn handle_execution_success(cmd: std::process::Child, logging_level: u8) -> () {
        match logging_level {
            0 => 
                println!("{:#?}", cmd.stdout),
            _ => 
                println!("{:#?}", cmd.stdout)
        }
    }

    pub fn handle_execution_error(error: std::io::Error, logging_level: u8) -> () {
        match logging_level {
            0 => 
                panic!("{:?}", error),
            _ => 
                panic!("{:?}", error)
        }
    }

}

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    #[test]
    fn test_assemble() {
        let manual_command = {
            Command::new("ls")
            .args(&["-l","-a"])
            .output()
            .expect("Failed to execute")
        };
        let test_command = {
            super::cmd_assemble::assemble("ls".to_string(), vec!("-l".to_string(),"-a".to_string()))
            .output()
            .expect("Failed to execute.")
        };

        assert_eq!(manual_command.stdout, test_command.stdout);
    }

    #[test]
    fn test_parse() {
        let expectation: (String, Vec<String>) = (
            "ls".to_string(),
            vec!("-l".to_string(), "-a".to_string())
        );
        let argument: Vec<String> = vec!(
            "ls".to_string(),
            "-l".to_string(),
            "-a".to_string()
        );

        assert_eq!(expectation, super::cmd_assemble::parse(argument));
    }

    #[test]
    fn test_parse_assemble() {
        let argument: Vec<String> = vec!(
            "ls".to_string(),
            "-l".to_string(),
            "-a".to_string()
        );
        let manual_assemble = {
            super::cmd_assemble::assemble("ls".to_string(), vec!("-l".to_string(),"-a".to_string()))
            .output()
            .expect("Failed to execute.")
        };

        let (arg1, arg2) = super::cmd_assemble::parse(argument);
        let test_assemble_with_parse = {
            super::cmd_assemble::assemble(arg1, arg2)
            .output()
            .expect("Failed to execute.")
        };

        assert_eq!(manual_assemble.stdout, test_assemble_with_parse.stdout);
    }
}