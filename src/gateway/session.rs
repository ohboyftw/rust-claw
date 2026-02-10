use crate::core::types::{AgentId, ChannelId, Session, SessionId};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_session(
        &self,
        channel_id: ChannelId,
        user_id: String,
        agent_id: AgentId,
    ) -> Result<Session> {
        // Simple session ID generation strategy for MVP
        let session_id = SessionId(format!("{}:{}", channel_id, user_id));
        let session = Session {
            id: session_id.clone(),
            channel_id,
            user_id,
            agent_id,
        };
        self.sessions
            .write()
            .unwrap()
            .insert(session_id, session.clone());
        Ok(session)
    }

    pub fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        self.sessions.read().unwrap().get(session_id).cloned()
    }

    pub fn find_session(&self, channel_id: &ChannelId, user_id: &str) -> Option<Session> {
        self.sessions
            .read()
            .unwrap()
            .values()
            .find(|s| s.channel_id == *channel_id && s.user_id == user_id)
            .cloned()
    }
}
