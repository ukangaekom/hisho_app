//! Platform-independent application logic shared by the CLI and frontends.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRequest {
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResponse {
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    EmptyCommand,
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCommand => formatter.write_str("command cannot be empty"),
        }
    }
}

impl std::error::Error for CoreError {}

pub fn execute(request: CommandRequest) -> Result<CommandResponse, CoreError> {
    let command = request.command.trim();

    if command.is_empty() {
        return Err(CoreError::EmptyCommand);
    }

    Ok(CommandResponse {
        output: format!("Executed: {command}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_a_command() {
        let response = execute(CommandRequest {
            command: "status".to_owned(),
        })
        .expect("status should execute");

        assert_eq!(response.output, "Executed: status");
    }

    #[test]
    fn rejects_empty_commands() {
        let result = execute(CommandRequest {
            command: "  ".to_owned(),
        });

        assert_eq!(result, Err(CoreError::EmptyCommand));
    }
}
