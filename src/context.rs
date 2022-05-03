use serde::{Deserialize, Serialize};

/// Keeps track of the current shell execution context
/// @attribute chdir : the current working directory
/// @attribute env : local environment variables set in the local context
/// @attribute profile : the current profile that is loaded by the shell
#[derive(Clone, Debug, Default)]
struct ExecContext<'a> {
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
    fn new(id: &'a str, chdir: &'a str, env: &'a [&'a str], profile: &'a str) -> Self {
        Self {
            id,
            chdir,
            env,
            profile,
        }
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
