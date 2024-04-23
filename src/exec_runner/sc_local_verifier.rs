use std::collections::BTreeMap;

use halo2_proofs::halo2curves::bn256::Fr;
use log::info;
use semaphore_aggregation::plonky2_verifier::verifier_api::std_ops;
use halo2_solidity_verifier::{compile_solidity, encode_calldata, revm, Evm};

use colored::Colorize;

pub struct SCLocalVerifier {
    pub verifier_address: revm::primitives::Address,
    pub vk_address_map: BTreeMap<usize, revm::primitives::Address>,
    evm: Evm
}

impl SCLocalVerifier {
    pub fn new(vk_vec: &Vec<usize>) -> Self {
        let mut evm = Evm::default();

        let verifier_solidity = std_ops::load_solidity(format!("verifier.sol")).expect(&format!("load `verifier.sol` error"));
        let verifier_code = compile_solidity(&verifier_solidity);
        let verifier_address = evm.create(verifier_code);

        let mut vk_address_map = BTreeMap::new();
        vk_vec.iter().for_each(|i| {
            let vk_solidity = std_ops::load_solidity(format!("{i}_vk.sol")).expect(&format!("load vk {i} error").red());
            let vk_code = compile_solidity(&vk_solidity);
            let vk_address = evm.create(vk_code);
            vk_address_map.insert(*i, vk_address);
        });

        Self { verifier_address, vk_address_map, evm }
    }

    pub fn verify_proof_locally_or_panic(&mut self, proof: &Vec<u8>, instances: &Vec<Fr>) {
        assert!(instances.len() % 4 == 0);
        let tx_num = instances.len() / 4 - 5;

        let vk_address = self.vk_address_map.get(&tx_num).unwrap();

        let calldata = encode_calldata(Some((*vk_address).into()), &proof, &instances);
        let (gas_cost, _output) = self.evm.call(self.verifier_address, calldata);
        info!("{}", format!("Gas cost: {}", gas_cost).yellow().bold());
    }
}