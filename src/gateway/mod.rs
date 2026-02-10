use crate::core::traits::{Agent, Channel};
use crate::core::types::{AgentId, ChannelEvent, ChannelId, Message, MessageType};
use crate::gateway::session::SessionManager;
use anyhow::{Context, Result};
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

pub mod session;

pub struct Gateway {
    channels: Arc<RwLock<HashMap<String, Arc<dyn Channel>>>>,
    agents: Arc<RwLock<HashMap<String, Arc<dyn Agent>>>>,
    session_manager: SessionManager,
    tx: mpsc::Sender<ChannelEvent>,
    rx: Option<mpsc::Receiver<ChannelEvent>>,
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}

impl Gateway {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            agents: Arc::new(RwLock::new(HashMap::new())),
            session_manager: SessionManager::new(),
            tx,
            rx: Some(rx),
        }
    }

    pub async fn register_channel(&self, channel: Arc<dyn Channel>) -> Result<()> {
        let id = channel.id();
        self.channels
            .write()
            .await
            .insert(id.clone(), channel.clone());

        let tx = self.tx.clone();
        let channel_clone = channel.clone();

        tokio::spawn(async move {
            if let Err(e) = channel_clone.start(tx).await {
                error!("Channel {} error: {:?}", id, e);
            }
        });

        info!("Registered channel: {}", channel.id());
        Ok(())
    }

    pub async fn register_agent(&self, agent: Arc<dyn Agent>) {
        let id = agent.id();
        self.agents.write().await.insert(id.clone(), agent.clone());
        info!("Registered agent: {}", id);
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut rx = self.rx.take().context("Gateway run called twice")?;
        info!("Gateway running...");

        while let Some(event) = rx.recv().await {
            match event {
                ChannelEvent::Message {
                    channel_id,
                    user_id,
                    content,
                } => {
                    if let Err(e) = self.handle_message(channel_id, user_id, content).await {
                        error!("Error handling message: {:?}", e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_message(
        &self,
        channel_id: ChannelId,
        user_id: String,
        content: String,
    ) -> Result<()> {
        info!(
            "Handling message from {} on channel {}",
            user_id, channel_id
        );

        // 1. Find or create session
        let session = if let Some(s) = self.session_manager.find_session(&channel_id, &user_id) {
            s
        } else {
            // Default agent selection logic
            let agents = self.agents.read().await;
            let agent_id = if let Some(first_agent) = agents.keys().next() {
                AgentId(first_agent.clone())
            } else {
                warn!("No agents registered!");
                return Ok(());
            };

            self.session_manager
                .create_session(channel_id.clone(), user_id.clone(), agent_id)?
        };

        // 2. Route to Agent
        let agent_opt = {
            let agents = self.agents.read().await;
            agents.get(&session.agent_id.0).cloned()
        };

        if let Some(agent) = agent_opt {
            let msg = Message {
                id: Uuid::new_v4().to_string(),
                session_id: session.id.clone(),
                sender: user_id,
                content,
                msg_type: MessageType::Text,
            };

            let response = agent.process(&session, &msg).await?;

            // 3. Route response back to Channel
            let channels = self.channels.read().await;
            if let Some(channel) = channels.get(&channel_id.0) {
                channel.send(response).await?;
            } else {
                error!("Channel {} not found to send response", channel_id);
            }
        } else {
            error!(
                "Agent {} not found for session {}",
                session.agent_id, session.id
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Message, Session};
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::sync::mpsc;

    struct MockChannel {
        id: String,
        sent_messages: Arc<Mutex<Vec<Message>>>,
    }

    #[async_trait]
    impl Channel for MockChannel {
        async fn start(&self, _tx: mpsc::Sender<ChannelEvent>) -> Result<()> {
            Ok(())
        }
        async fn send(&self, msg: Message) -> Result<()> {
            self.sent_messages.lock().await.push(msg);
            Ok(())
        }
        fn id(&self) -> String {
            self.id.clone()
        }
    }

    struct MockAgent {
        id: String,
    }

    #[async_trait]
    impl Agent for MockAgent {
        async fn process(&self, session: &Session, msg: &Message) -> Result<Message> {
            Ok(Message {
                id: "resp".to_string(),
                session_id: session.id.clone(),
                sender: "agent".to_string(),
                content: format!("Echo: {}", msg.content),
                msg_type: MessageType::Text,
            })
        }
        fn id(&self) -> String {
            self.id.clone()
        }
    }

    #[tokio::test]
    async fn test_gateway_routing() {
        let mut gateway = Gateway::new();
        let sent_msgs = Arc::new(Mutex::new(Vec::new()));

        let channel = Arc::new(MockChannel {
            id: "test-chan".to_string(),
            sent_messages: sent_msgs.clone(),
        });

        let agent = Arc::new(MockAgent {
            id: "test-agent".to_string(),
        });

        gateway.register_channel(channel.clone()).await.unwrap();
        gateway.register_agent(agent).await;

        // Simulate incoming message by pushing directly to tx (bypassing start)
        // Or we can use `handle_message` directly if it was public.
        // Or we can trigger it via the channel start mechanism.
        // Let's just send to gateway.tx manually? No, tx is private.
        // But `register_channel` gave `tx` to `channel.start`.
        // MockChannel::start does nothing.

        // Wait, handle_message is private.
        // I should expose a method to inject message for testing or use the channel's capability.

        // Let's manually trigger `handle_message`? It's private.
        // I can change MockChannel to keep the tx and send a message.

        // BUT, `gateway.run()` blocks. I need to run it in a task.
        let tx = gateway.tx.clone();

        tokio::spawn(async move {
            gateway.run().await.unwrap();
        });

        // Send a message
        tx.send(ChannelEvent::Message {
            channel_id: ChannelId("test-chan".to_string()),
            user_id: "user1".to_string(),
            content: "hello".to_string(),
        })
        .await
        .unwrap();

        // Give it some time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msgs = sent_msgs.lock().await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Echo: hello");
    }
}
