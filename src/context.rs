#![warn(dead_code)]
use serde::Serialize;
use std::env;
use std::io;
use std::path::Path;

/// Keeps track of the current shell execution context
/// @attribute chdir : the current working directory
/// @attribute env : local environment variables set in the local context
/// @attribute profile : the current profile that is loaded by the shell
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExecContext<'a> {
    id: &'a str,
    chdir: &'a str,
    env: &'a [&'a str],
    profile: &'a str,
}

impl<'a> ExecContext<'a> {
    /// Creates a new execContext
    /// @param chdir : the current working directory
    /// @param env : local environment variables set in the local context
    /// @param profile : the current profile that is loaded by the shell
    pub fn new(id: &'a str, chdir: &'a str, env: &'a [&'a str], profile: &'a str) -> Self {
        Self {
            id,
            chdir,
            env,
            profile,
        }
    }

    /// Check if the current shell context needs to be reloaded due to errors or updates
    /// @returns true if we hit an error when querying the working directory
    ///     or the profile has changed or working directory doesn't match current context
    ///
    ///
    pub fn requires_reload(&self) -> bool {
        let workdir = match env::current_dir() {
            Err(e) => {
                println!("{}", e);
                return true;
            }
            Ok(dir) => dir,
        };

        let workdir = match workdir.to_str() {
            None => {
                println!("Failed to get current working directory");
                return true;
            }
            Some(dir) => dir,
        };

        self.profile != "default" && self.chdir != workdir
    }

    // Executes a single command in the context
    // Assumes there is only one executable unit :
    // i.e. no execution flow and multilpe commands "|", "||", "&&"
    // @param input : the "unit" command to execute
    pub fn exec(&mut self, input: &'a str) -> io::Result<()> {
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir: &'a str = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                match env::set_current_dir(&root) {
                    Ok(_) => self.chdir = new_dir,
                    Err(e) => return Err(e),
                }
            }
            command => {
                println!("Running {} command, not implemented yet", command);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_context_new() {
        let id = "test";
        let chdir = "/home/test";
        let env = ["test=test"];
        let profile = "test";
        let context = ExecContext::new(id, chdir, &env, profile);
        assert_eq!(context.id, id);
        assert_eq!(context.chdir, chdir);
        assert_eq!(context.env, &env);
        assert_eq!(context.profile, profile);
    }
}
