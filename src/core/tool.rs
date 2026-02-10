use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the name of the tool (unique identifier).
    fn name(&self) -> String;

    /// Returns a description of what the tool does.
    fn description(&self) -> String;

    /// Executes the tool with the given arguments.
    async fn execute(&self, args: Value) -> Result<String>;
}

pub struct TimeTool;

#[async_trait]
impl Tool for TimeTool {
    fn name(&self) -> String {
        "get_current_time".to_string()
    }

    fn description(&self) -> String {
        "Returns the current UTC time as a string.".to_string()
    }

    async fn execute(&self, _args: Value) -> Result<String> {
        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        Ok(datetime.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_time_tool() {
        let tool = TimeTool;
        assert_eq!(tool.name(), "get_current_time");
        let result = tool.execute(serde_json::Value::Null).await.unwrap();
        assert!(!result.is_empty());
        // Simple check for date format (starts with year)
        assert!(result.chars().next().unwrap().is_numeric());
    }
}
