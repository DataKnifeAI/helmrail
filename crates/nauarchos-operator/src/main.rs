//! Nauarchos operator entrypoint.
//!
//! M0: boots and logs identity. M1+: kube-rs reconcile loops for AgentClass,
//! Agent, AgentFleet, AgentTask, SandboxProfile, DependencyBundle.

use nauarchos_crd::{API_GROUP, API_VERSION};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!(
        group = API_GROUP,
        version = API_VERSION,
        "nauarchos operator bootstrap — reconcile loops not yet implemented (see docs/PLAN.md)"
    );

    // Keep process idle-friendly for local runs; k8s Deployment will restart on exit.
    tokio::signal::ctrl_c().await?;
    tracing::info!("shutdown signal received");
    Ok(())
}
