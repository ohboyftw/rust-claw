use anyhow::Result;
use app::agent::echo::EchoAgent;
use app::agent::llm::LLMAgent;
use app::channels::console::ConsoleChannel;
use app::gateway::Gateway;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let gateway = Gateway::new();

    // Register Channels
    let console = Arc::new(ConsoleChannel);
    gateway.register_channel(console).await?;

    // Register Agents
    let echo_agent = Arc::new(EchoAgent);
    gateway.register_agent(echo_agent).await;

    let llm_agent = Arc::new(LLMAgent::new("gpt".to_string(), None));
    gateway.register_agent(llm_agent).await;

    println!("OpenClaw Rust Gateway running...");
    println!("Press Ctrl+C to stop.");

    tokio::select! {
        res = gateway.run() => {
            if let Err(e) = res {
                eprintln!("Gateway error: {:?}", e);
            }
        }
        _ = signal::ctrl_c() => {
            println!("Shutting down...");
        }
    }

    Ok(())
}
