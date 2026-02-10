use crate::core::traits::Channel;
use crate::core::types::{ChannelEvent, ChannelId, Message};
use anyhow::Result;
use async_trait::async_trait;
use log::error;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;

pub struct ConsoleChannel;

#[async_trait]
impl Channel for ConsoleChannel {
    async fn start(&self, event_sender: Sender<ChannelEvent>) -> Result<()> {
        println!("Console Channel Started. Type something:");
        let mut stdin = BufReader::new(io::stdin());
        let mut line = String::new();

        loop {
            line.clear();
            match stdin.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let content = line.trim().to_string();
                    if content.is_empty() {
                        continue;
                    }

                    if let Err(e) = event_sender
                        .send(ChannelEvent::Message {
                            channel_id: ChannelId("console".to_string()),
                            user_id: "user".to_string(),
                            content,
                        })
                        .await
                    {
                        error!("Failed to send console event: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Error reading stdin: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn send(&self, message: Message) -> Result<()> {
        println!("Bot: {}", message.content);
        Ok(())
    }

    fn id(&self) -> String {
        "console".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{AgentId, MessageType, SessionId};

    #[tokio::test]
    async fn test_console_channel_id() {
        let channel = ConsoleChannel;
        assert_eq!(channel.id(), "console");
    }

    #[tokio::test]
    async fn test_console_channel_send() {
        let channel = ConsoleChannel;
        let msg = Message {
            id: "1".to_string(),
            session_id: SessionId("s1".to_string()),
            sender: "agent".to_string(),
            content: "hello".to_string(),
            msg_type: MessageType::Text,
        };
        // This just prints to stdout, difficult to assert content, but verifies no panic.
        assert!(channel.send(msg).await.is_ok());
    }
}
