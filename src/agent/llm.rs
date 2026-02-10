use crate::core::traits::Agent;
use crate::core::types::{Message, MessageType, Session};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct LLMAgent {
    pub id: String,
    pub api_key: Option<String>,
}

impl LLMAgent {
    pub fn new(id: String, api_key: Option<String>) -> Self {
        Self { id, api_key }
    }
}

#[async_trait]
impl Agent for LLMAgent {
    async fn process(&self, session: &Session, message: &Message) -> Result<Message> {
        let response_text = if let Some(_key) = &self.api_key {
            // In a real impl, we would call an API
            format!("LLM says: {}", message.content)
        } else {
            "I am a basic LLM mock. I don't have an API key yet.".to_string()
        };

        Ok(Message {
            id: Uuid::new_v4().to_string(),
            session_id: session.id.clone(),
            sender: self.id.clone(),
            content: response_text,
            msg_type: MessageType::Text,
        })
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{AgentId, ChannelId, SessionId};

    #[tokio::test]
    async fn test_llm_agent_mock() {
        let agent = LLMAgent::new("gpt-mock".to_string(), None);
        let session = Session {
            id: SessionId("s1".to_string()),
            channel_id: ChannelId("c1".to_string()),
            user_id: "u1".to_string(),
            agent_id: AgentId("gpt-mock".to_string()),
        };
        let msg = Message {
            id: "m1".to_string(),
            session_id: session.id.clone(),
            sender: "u1".to_string(),
            content: "hello".to_string(),
            msg_type: MessageType::Text,
        };

        let response = agent.process(&session, &msg).await.unwrap();
        assert!(response.content.contains("I am a basic LLM mock"));
    }
}
