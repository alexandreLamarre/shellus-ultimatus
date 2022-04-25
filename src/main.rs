use std::env;
use std::io::Write;
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    println!("{}", e);
                }
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            command => {
                let mut child = std::process::Command::new(command).args(args).spawn();

                // gracefully handle errors
                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }

    Ok(())
}
