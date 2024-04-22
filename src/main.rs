use exec_system::{runtime::RuntimeConfig, traits::EnvConfig};
use log::info;

use colored::Colorize;
use zk_6358_runner::exec_runner::testnet_executor::TestnetExecutor;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    exec_system::initiallize::sys_env_init("./.config/sys.config");
    exec_system::initiallize::sys_log_init(Some(vec!["zk_6358_runner".to_string(), "zk_6358_prover".to_string(), "plonky2::util::timing".to_string(), "crypto".to_string(), "interact".to_string()]));

    let runtime_config = RuntimeConfig::from_env();

    info!("{}", format!("start {}", runtime_config.network).green().bold());

    let mut runtime_exec = TestnetExecutor::new("./object-store").await;
    runtime_exec.load_current_state_from_local("./test-data").await.unwrap();

    // runtime_exec.try_execute_one_batch(4).await?;
    runtime_exec.try_execute_one_batch(8).await
}
