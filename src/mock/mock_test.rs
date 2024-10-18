#[cfg(test)]
mod tests {
    use log::{info, LevelFilter};
    use plonky2::{fri::FriConfig, iop::witness::{PartialWitness, WitnessWrite}, plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitConfig, config::{GenericConfig, GenericHashOut, PoseidonGoldilocksConfig}}};
    use plonky2::field::types::{Sample, Field};
    use plonky2_ecdsa::gadgets::recursive_proof::recursive_proof_2;

    #[test]
    fn arbitrary_circuiit() {
        let mut log_builder = env_logger::Builder::from_default_env();
        log_builder.format_timestamp(None);
        log_builder.filter_level(LevelFilter::Info);
        let _ = log_builder.try_init();

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();
        let mut pw: PartialWitness<F> = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let targets_num = 4 + 4 * 1024;

        let dummy_targets = builder.add_virtual_targets(targets_num);
        let mut sum_v = F::ZERO;
        let mut sum_t = builder.zero();
        dummy_targets.iter().for_each(|&tar| {
            let r_t = F::rand();
            pw.set_target(tar, r_t);
            sum_v += r_t;
            sum_t = builder.add(sum_t, tar);
        });

        builder.register_public_inputs(&dummy_targets);

        let expected_sum_t = builder.add_virtual_target();
        pw.set_target(expected_sum_t, sum_v);
        builder.connect(expected_sum_t, sum_t);

        let data = builder.build::<C>();
        let proof = data.prove(pw).expect("prove error!");
        data.verify(proof.clone()).expect("verify error!");
        info!("complete basic circuit");

        let config = CircuitConfig::standard_recursion_config();
        let middle_proof = recursive_proof_2::<F, C, C, D>(&vec![(proof, data.verifier_only, data.common)], &config, None).unwrap();

        info!("complete middle circuit");

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
            recursive_proof_2::<F, C, C, D>(&vec![middle_proof], &high_rate_config, None).unwrap();

        info!("circuit digest of `final_proof`: {:?}", final_proof.1.circuit_digest.to_bytes());
    }
}