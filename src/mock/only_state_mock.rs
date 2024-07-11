use exec_system::traits::EnvConfig;
use log::info;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use zk_6358_prover::circuit::state_prover::ZK6358StateProverEnv;


const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type H = <C as GenericConfig<D>>::Hasher;

pub struct OnlyStateProverMock
{
    runtime_zk_prover: ZK6358StateProverEnv<H, F, D>,
}

impl OnlyStateProverMock {
    pub async fn new() -> Self {
        let db_config = exec_system::database::DatabaseConfig::from_env();

        info!("{:?}", db_config.smt_kv);

        Self { 
            runtime_zk_prover: ZK6358StateProverEnv::<H, F, D>::new(&db_config.smt_kv).await,
        }
    }

    // #[cfg(feature = "mocktest")]
    // pub fn hello(&self) {
    //     info!("hello!");
    //     self.runtime_zk_prover;
    //     hello
    // }
}