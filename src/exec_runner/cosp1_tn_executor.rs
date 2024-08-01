use exec_system::traits::EnvConfig;
use log::info;
use plonky2::{
    fri::FriConfig, 
    plonk::{circuit_data::CircuitConfig, config::{GenericConfig, PoseidonGoldilocksConfig}}
};
use plonky2_ecdsa::gadgets::recursive_proof::{recursive_proof_2, ProofTuple};
use zk_6358::utils6358::transaction::GasFeeTransaction;
use zk_6358_prover::{circuit::{state_prover::ZK6358StateProverEnv, zk6358_recursive_proof::zk_6358_chunked_state_recursive_proof}, types::signed_tx_types::SignedOmniverseTx};

use anyhow::Result;

use crate::strategy::circuit_runtime::OnlyStateProverCircuitRT;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type H = <C as GenericConfig<D>>::Hasher;

pub struct CoSP1TestnetExecutor
{
    runtime_zk_prover: ZK6358StateProverEnv<H, F, D>,
}

impl CoSP1TestnetExecutor {
    const BATCH_SIZE: usize = 128;

    pub async fn new() -> Self {
        let db_config = exec_system::database::DatabaseConfig::from_env();

        info!("{:?}", db_config.smt_kv);

        Self { 
            runtime_zk_prover: ZK6358StateProverEnv::<H, F, D>::new(&db_config.smt_kv).await,
        }
    }

    #[cfg(feature = "mocktest")]
    pub async fn p_test_init_gas_inputs(&mut self, gas_tx_vec: &Vec<GasFeeTransaction>) {
        for gas_tx in gas_tx_vec.iter() {
            self.runtime_zk_prover.init_utxo_inputs(&gas_tx.generate_inputs_utxo()).await;
        }
    }

    pub async fn execute_one_batch(&mut self, batched_somtx_vec: &Vec<SignedOmniverseTx>) -> Result<ProofTuple<F, C, D>> {
        let mut rzp_branch = self.runtime_zk_prover.fork();

        let batched_proof = rzp_branch.state_only_crt_prove::<C>(batched_somtx_vec).await?;

        // remember to flush to db, or the local state will not be updated
        self.runtime_zk_prover.merge(rzp_branch);
        self.runtime_zk_prover.flush_state_after_final_verification().await;

        Ok(batched_proof)
    }

    pub async fn exec_state_prove_circuit(&mut self, somtx_container: &Vec<SignedOmniverseTx>) -> Result<ProofTuple<F, C, D>> {
        assert_eq!(somtx_container.len() % Self::BATCH_SIZE, 0, "Invalid `somtx_container` size");
        
        let mut rzp_branch = self.runtime_zk_prover.fork();

        let mut batched_proofs = Vec::new();
        for (i, batched_somtx_vec) in somtx_container.chunks(Self::BATCH_SIZE).enumerate() {
            info!("processing batch: {}", i);

            batched_proofs.push(rzp_branch.state_only_crt_prove::<C>(&batched_somtx_vec).await?);
        }

        // let middle_proof = rzp_branch.state_only_crt_prove::<C>(batched_somtx_vec).await?;

        let config = CircuitConfig::standard_recursion_config();

        // to be parallized
        let middle_proof =
            zk_6358_chunked_state_recursive_proof(&batched_proofs, &config.clone(), None)
                .unwrap();

        let standard_config = CircuitConfig::standard_recursion_config();
        let high_rate_config = CircuitConfig {
            fri_config: FriConfig {
                rate_bits: 7,
                proof_of_work_bits: 16,
                num_query_rounds: 12,
                ..standard_config.fri_config.clone()
            },
            ..standard_config
        };

        let final_proof =
            recursive_proof_2::<F, C, C, D>(&vec![middle_proof], &high_rate_config, None)?;

        // remember to flush to db, or the local state will not be updated
        self.runtime_zk_prover.merge(rzp_branch);
        self.runtime_zk_prover.flush_state_after_final_verification().await;

        Ok(final_proof)
    }
}

#[cfg(feature = "mocktest")]
pub async fn state_only_mocking() {
    use crate::mock::mock_utils::mock_on::p_test_generate_a_batch;
    use plonky2_ecdsa::curve::{curve_types::{AffinePoint, CurveScalar, Curve}, ecdsa::{ECDSAPublicKey, ECDSASecretKey}, secp256k1::Secp256K1};
    use plonky2::{field::{secp256k1_scalar::Secp256K1Scalar, types::Sample}, util::timing::TimingTree};
    use itertools::Itertools;
    use log::Level;
    use circuit_local_storage::circuit::p_v_io::proof_tuple_to_local;

    type EC = Secp256K1;

    info!("start mock testing for utxo state");

    let sk = ECDSASecretKey::<EC>(Secp256K1Scalar::rand());
    let pk = ECDSAPublicKey::<EC>((CurveScalar(sk.0) * EC::GENERATOR_PROJECTIVE).to_affine());
    let AffinePoint { x, y, .. } = pk.0;
    let mut x_le_bytes = Vec::new();
    x.0.iter().for_each(|i| {
        x_le_bytes.append(&mut i.to_le_bytes().to_vec());
    });
    x_le_bytes.reverse();

    let mut y_le_bytes = Vec::new();
    y.0.iter().for_each(|i| {
        y_le_bytes.append(&mut i.to_le_bytes().to_vec());
    });

    // let mut rl = rustyline::DefaultEditor::new().unwrap();

    // let o_s_line = rl.readline(">>input batch count(one batch 4 txs): ").unwrap();
    // let tx_n: usize = usize::from_str_radix(&o_s_line, 10).unwrap();

    let total_timing = TimingTree::new("total processing time.", Level::Info);
    let tx_n = 128;

    let mut batched_somtx_vec = Vec::new();
    (0..tx_n / 4).for_each(|_| {
        batched_somtx_vec.append(&mut p_test_generate_a_batch(
            sk,
            x_le_bytes.clone().try_into().unwrap(),
            y_le_bytes.clone().try_into().unwrap(),
        ));
    });

    let test_gas_tx_vec = batched_somtx_vec
        .iter()
        .map(|somtx| somtx.borrow_gas_transaction().clone())
        .collect_vec();

    let mut cosp1_executor = CoSP1TestnetExecutor::new().await;
    let timing = TimingTree::new("initializing state circuit", Level::Info);
    cosp1_executor.p_test_init_gas_inputs(&test_gas_tx_vec).await;
    timing.print();

    let final_proof = cosp1_executor.exec_state_prove_circuit(&batched_somtx_vec).await.expect("mock state proving error");
    total_timing.print();

    proof_tuple_to_local(&format!("{}-tx", tx_n), &final_proof, false).unwrap();
    proof_tuple_to_local(&format!("{}-tx", tx_n), &final_proof, true).unwrap();
}
