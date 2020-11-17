mod cmd_assemble {
    use std::process::Command;

    pub fn build_command(cmd: &str, args: Vec<&str>) -> Command {
        let mut ret: Command = Command::new(cmd);
        ret.args(args);
    
        ret
    }
}

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    #[test]
    fn test_build_command() {
        let manual_command = {
            Command::new("ls")
            .args(&["-l","-a"])
            .output()
            .expect("Failed to execute")
        };
        let test_command = {
            super::cmd_assemble::build_command("ls", vec!("-l","-a"))
            .output()
            .expect("Failed to execute.")
        };

        assert_eq!(manual_command.stdout, test_command.stdout);
    }
}