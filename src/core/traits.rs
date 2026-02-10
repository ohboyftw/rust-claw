use crate::core::types::{ChannelEvent, Message, Session};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Channel: Send + Sync {
    /// Starts the channel listening loop.
    /// It sends events to the gateway via `event_sender`.
    async fn start(&self, event_sender: Sender<ChannelEvent>) -> Result<()>;

    /// Sends a message out to the channel.
    async fn send(&self, message: Message) -> Result<()>;

    /// Returns the channel ID.
    fn id(&self) -> String;
}

#[async_trait]
pub trait Agent: Send + Sync {
    /// Processes an incoming message and returns a response.
    async fn process(&self, session: &Session, message: &Message) -> Result<Message>;

    /// Returns the agent ID.
    fn id(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{AgentId, ChannelId, MessageType, SessionId};

    struct MockChannel;
    #[async_trait]
    impl Channel for MockChannel {
        async fn start(&self, _tx: Sender<ChannelEvent>) -> Result<()> {
            Ok(())
        }
        async fn send(&self, _msg: Message) -> Result<()> {
            Ok(())
        }
        fn id(&self) -> String {
            "mock".to_string()
        }
    }

    struct MockAgent;
    #[async_trait]
    impl Agent for MockAgent {
        async fn process(&self, session: &Session, msg: &Message) -> Result<Message> {
            Ok(Message {
                id: "response".to_string(),
                session_id: session.id.clone(),
                sender: "agent".to_string(),
                content: msg.content.clone(),
                msg_type: MessageType::Text,
            })
        }
        fn id(&self) -> String {
            "mock_agent".to_string()
        }
    }

    #[tokio::test]
    async fn test_mock_implementations() {
        let channel = MockChannel;
        let agent = MockAgent;
        let (tx, _rx) = tokio::sync::mpsc::channel(1);

        assert!(channel.start(tx).await.is_ok());
        assert_eq!(channel.id(), "mock");

        let session = Session {
            id: SessionId("s1".to_string()),
            channel_id: ChannelId("c1".to_string()),
            user_id: "u1".to_string(),
            agent_id: AgentId("a1".to_string()),
        };
        let msg = Message {
            id: "m1".to_string(),
            session_id: session.id.clone(),
            sender: "u1".to_string(),
            content: "hi".to_string(),
            msg_type: MessageType::Text,
        };

        let response = agent.process(&session, &msg).await.unwrap();
        assert_eq!(response.content, "hi");
    }
}
