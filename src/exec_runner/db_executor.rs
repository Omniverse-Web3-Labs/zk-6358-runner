use cached_smt_db::db::db_live::SMTDBLive;
use exec_system::{database::DatabaseConfig, traits::EnvConfig};

use anyhow::Result;
use colored::Colorize;

pub struct DBExecutor {
    pub db_config: DatabaseConfig
}

impl DBExecutor {
    pub fn new() -> Self {
        Self {
            db_config: DatabaseConfig::from_env()
        }
    }

    pub async fn reset_smt_db(&self) -> Result<()>{
        let mut smt_db_live = SMTDBLive::new(&self.db_config.smt_url).await;

        smt_db_live.clear_all_data().await?;

        Ok(())
    }
}

pub async fn run_db_exec() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    let dbline = rl.readline(">>input db type(`smt-db`|`remote-db`): ").unwrap();
    // match readline {
    //     Ok(line) => {
    //         rl.add_history_entry(line.as_str()).unwrap();
    //         println!("Line: {line}");
    //     }
    //     Err(ReadlineError::Interrupted) => {
    //         println!("Interrupted");
    //     }
    //     Err(ReadlineError::Eof) => {
    //         println!("Encountered Eof");
    //     }
    //     Err(err) => {
    //         println!("Error: {err:?}");
    //     }
    // }

    match dbline.as_str() {
        "smt-db" => {
            let db_exec = DBExecutor::new();

            let op_line = rl.readline(">>input operation type(`reset`): ").unwrap();
            match op_line.as_str() {
                "reset" => {
                    db_exec.reset_smt_db().await.unwrap();
                },
                _ => { panic!("{}", format!("invalid op type {op_line}. expected `reset`").red().bold()); }
            }
        },
        _=> { panic!("{}", format!("invalid db type {dbline}. expcted `smt-db` or `remote-db`").red().bold()); }
    }
}
