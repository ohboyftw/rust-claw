# Requirements for Rust-based OpenClaw Alternative

## Overview
Create a high-performance, memory-safe alternative to OpenClaw using Rust. The system will function as a personal AI assistant gateway, connecting various messaging channels to an AI agent.

## Core Architecture
The system shall follow a Hub-and-Spoke architecture similar to OpenClaw's Gateway.

### 1. Gateway (Control Plane)
- **Responsibility:** Central message router and state manager.
- **Features:**
    - **Session Management:** Maintain active conversational sessions for users across different channels.
    - **Message Routing:** Route incoming messages from Channels to the appropriate Agent Session.
    - **Response Routing:** Route agent responses back to the originating Channel.
    - **Configuration:** Load settings from configuration files (e.g., TOML/JSON) or environment variables.
    - **Concurrency:** Handle multiple channels and sessions concurrently using async Rust (Tokio).

### 2. Core Interfaces (Traits)
- **`Channel` Interface:**
    - Must provide a method to start listening for incoming events.
    - Must provide a method to send outgoing messages (text, attachments).
    - Examples: Console (STDIN/OUT), Telegram, Discord, WebSocket (for WebChat).
- **`Agent` Interface:**
    - Must accept incoming context/messages.
    - Must produce responses (text, tool calls).
    - Abstraction over LLM providers (e.g., Anthropic, OpenAI).
- **`Tool` Interface:**
    - Standardized way to define executable functions that the Agent can invoke.

### 3. Subsystems
- **Session Manager:**
    - Stores conversation history (in-memory initially, persistent later).
    - Handles context window management (truncation/summarization).
- **LLM Client:**
    - A module to communicate with AI Model APIs (Anthropic/OpenAI).
    - Support for streaming responses.

### 4. MVP Scope (for this task)
1.  **Core Gateway:** Implementation of the event loop and routing logic.
2.  **Interfaces:** Rust Traits for `Channel` and `Agent`.
3.  **Implementations:**
    - **Console Channel:** A channel that reads from STDIN and writes to STDOUT for easy testing.
    - **Echo Agent:** A simple agent that echoes input (to verify routing).
    - **Basic LLM Agent:** An agent capable of a simple chat using an external API (mocked if no API key provided).
4.  **Tests:** Unit tests for routing and integration tests for the full flow.

## Functional Requirements
1.  **Startup:** System starts, loads config, initializes enabled channels.
2.  **Message Ingestion:** System receives a message "Hello" from Console Channel.
3.  **Session Creation:** System identifies the user, creates/retrieves a session.
4.  **Processing:** Agent processes the message.
5.  **Response:** Agent sends "Hi there" back to the Console Channel.
6.  **Tool Use (Optional but good):** Agent decides to call a tool (e.g., `time`).

## Non-Functional Requirements
- **Language:** Rust (2021 edition or later).
- **Async Runtime:** Tokio.
- **Error Handling:** Robust error handling (Result/Option) without crashing the gateway.
- **Testing:** High test coverage for core logic.
