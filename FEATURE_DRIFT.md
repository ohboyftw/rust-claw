# Feature Drift Analysis: OpenClaw vs Rust Implementation

This document outlines the feature gaps (drifts) between the original Node.js/TypeScript-based [OpenClaw](https://github.com/openclaw/openclaw) and this Rust-based MVP implementation.

## 1. Core Architecture
- **OpenClaw:** Full-featured Gateway with WebSocket control plane, RPC for Pi agent runtime, and robust session management (pruning, summarization).
- **Rust MVP:** Basic Gateway with in-memory `SessionManager`, async trait-based `Channel`/`Agent` interfaces, and simple message routing.
- **Drift:**
    - No persistent storage for sessions.
    - No RPC mechanism for remote agents/skills.
    - No context window management or summarization logic.
    - Missing the "Pi agent runtime" concept (currently agents are compiled-in structs).

## 2. Channels
- **OpenClaw:** Supports 10+ platforms including WhatsApp, Telegram, Slack, Discord, Signal, iMessage (BlueBubbles), etc.
- **Rust MVP:** Implements only a `ConsoleChannel` (STDIN/STDOUT) for local testing.
- **Drift:** Missing all external messaging platform integrations. The `Channel` trait is generic enough to support them, but the implementations are absent.

## 3. Agents & LLM Integration
- **OpenClaw:** First-class support for Anthropic (Claude) and OpenAI (GPT), with model failover and advanced prompting strategies (e.g., "Thinking" modes).
- **Rust MVP:** `EchoAgent` (for testing) and a mock `LLMAgent` that returns static strings.
- **Drift:**
    - No real HTTP client integration with LLM providers.
    - No prompt engineering or template system.
    - No streaming response support (currently returns full `Message`).

## 4. Tools & Skills
- **OpenClaw:** Extensive tool ecosystem: Browser control (Puppeteer/CDP), Canvas (A2UI), System tools (screen recording, camera), and a Skill Registry (ClawHub).
- **Rust MVP:** Introduced `Tool` trait and a basic `TimeTool`.
- **Drift:**
    - The architectural interface exists (`src/core/tool.rs`), but no complex tools (browser, canvas) are implemented.
    - No registry or discovery mechanism (ClawHub) for tools.
    - Agents are not yet wired to *use* these tools automatically (LLM function calling loop is missing).

## 5. User Interfaces & Clients
- **OpenClaw:** Web Dashboard (Control UI), macOS Menu Bar App, Mobile Nodes (iOS/Android).
- **Rust MVP:** CLI-based entry point only.
- **Drift:** No graphical user interface or remote management capabilities.

## 6. Deployment & Operations
- **OpenClaw:** Docker support, Nix support, Tailscale (Serve/Funnel) integration for remote access.
- **Rust MVP:** Standard Rust binary (`cargo run`).
- **Drift:** Missing containerization, remote access tunnels, and advanced configuration management.

## 7. Safety & Sandboxing
- **OpenClaw:** Docker-based sandboxing for executing untrusted code/tools.
- **Rust MVP:** Runs with full user privileges; no sandboxing.
- **Drift:** Critical security features for executing arbitrary code are missing.

## Summary
The Rust implementation successfully establishes the *architectural foundation* (the "Gateway" pattern with channels and agents) but currently lacks the rich ecosystem of integrations, tools, and operational features that make OpenClaw a production-ready assistant.
