use crate::core::traits::Agent;
use crate::core::types::{Message, MessageType, Session};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct EchoAgent;

#[async_trait]
impl Agent for EchoAgent {
    async fn process(&self, session: &Session, message: &Message) -> Result<Message> {
        Ok(Message {
            id: Uuid::new_v4().to_string(),
            session_id: session.id.clone(),
            sender: "echo-agent".to_string(),
            content: format!("Echo: {}", message.content),
            msg_type: MessageType::Text,
        })
    }

    fn id(&self) -> String {
        "echo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{AgentId, ChannelId, SessionId};

    #[tokio::test]
    async fn test_echo_agent() {
        let agent = EchoAgent;
        let session = Session {
            id: SessionId("s1".to_string()),
            channel_id: ChannelId("c1".to_string()),
            user_id: "u1".to_string(),
            agent_id: AgentId("echo".to_string()),
        };
        let msg = Message {
            id: "m1".to_string(),
            session_id: session.id.clone(),
            sender: "u1".to_string(),
            content: "hello".to_string(),
            msg_type: MessageType::Text,
        };

        let response = agent.process(&session, &msg).await.unwrap();
        assert_eq!(response.content, "Echo: hello");
    }
}
