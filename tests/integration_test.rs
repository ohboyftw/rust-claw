use anyhow::Result;
use app::agent::echo::EchoAgent;
use app::core::traits::Channel;
use app::core::types::{ChannelEvent, ChannelId, Message};
use app::gateway::Gateway;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

struct MockChannel {
    id: String,
    received: Arc<Mutex<Vec<Message>>>,
}

#[async_trait]
impl Channel for MockChannel {
    async fn start(&self, tx: mpsc::Sender<ChannelEvent>) -> Result<()> {
        // Send a test message
        tx.send(ChannelEvent::Message {
            channel_id: ChannelId("mock-channel".to_string()),
            user_id: "test-user".to_string(),
            content: "ping".to_string(),
        })
        .await
        .unwrap();
        Ok(())
    }

    async fn send(&self, msg: Message) -> Result<()> {
        self.received.lock().await.push(msg);
        Ok(())
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

#[tokio::test]
async fn test_end_to_end_flow() {
    // 1. Setup Gateway
    let mut gateway = Gateway::new();

    // 2. Register Mock Channel
    let received_msgs = Arc::new(Mutex::new(Vec::new()));
    let channel = Arc::new(MockChannel {
        id: "mock-channel".to_string(),
        received: received_msgs.clone(),
    });
    gateway.register_channel(channel.clone()).await.unwrap();

    // 3. Register Echo Agent
    let agent = Arc::new(EchoAgent);
    gateway.register_agent(agent).await;

    // 4. Run Gateway in background
    tokio::spawn(async move {
        gateway.run().await.unwrap();
    });

    // 5. Wait for processing (with timeout)
    let start = std::time::Instant::now();
    loop {
        let msgs = received_msgs.lock().await;
        if !msgs.is_empty() {
            assert_eq!(msgs[0].content, "Echo: ping");
            break;
        }
        if start.elapsed().as_secs() > 2 {
            panic!("Timeout waiting for response");
        }
        drop(msgs); // Release lock before sleep
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
