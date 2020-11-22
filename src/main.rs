mod cli;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let (cmd, args) = cli::cmd_assemble::parse(arguments);
    let mut command = cli::cmd_assemble::assemble(cmd, args);
}
