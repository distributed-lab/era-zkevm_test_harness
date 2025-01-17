use self::run_manually::{
    run_and_try_create_witness_for_extended_state, run_and_try_create_witness_inner, Options,
};
use super::*;
use crate::tests::utils::preprocess_asm::preprocess_asm;
pub use crate::tests::utils::preprocess_asm::TemplateDictionary;
use std::{fs, path::Path};
use zkevm_assembly::Assembly;

/// Runs the tests based on the ASM files from a given directory.
/// The main assembly should be in `entry.asm` file, while additional
/// contracts should be in `ADDRESS.asm` files, where `ADDRESS` is the numerical
/// address at which they should be deployed.
pub fn run_asm_based_test(test_dir: &str, additional_contracts: &[i32], options: Options) {
    run_asm_based_test_template(test_dir, additional_contracts, options, None);
}

pub fn run_asm_based_test_template(
    test_dir: &str,
    additional_contracts: &[i32],
    options: Options,
    dictionary: Option<&TemplateDictionary>,
) {
    let data_path = Path::new(test_dir);

    let contracts: Vec<(H160, Vec<[u8; 32]>)> = additional_contracts
        .iter()
        .map(|address| {
            let bytecode = compile_asm_template(data_path, &address.to_string(), dictionary, None);
            (Address::from_low_u64_be(*address as u64), bytecode)
        })
        .collect();

    let entry_bytecode = compile_asm_template(data_path, "entry", dictionary, Some(&contracts));

    let mut options = options.clone();
    options.other_contracts = contracts;
    run_with_options(entry_bytecode, options);
}

fn compile_asm_template(
    data_path: &Path,
    filename: &str,
    dictionary: Option<&TemplateDictionary>,
    additional_contracts: Option<&Vec<(H160, Vec<[u8; 32]>)>>,
) -> Vec<[u8; 32]> {
    let file_path = data_path.join(format!("{filename}.asm"));
    let asm = fs::read_to_string(file_path.clone()).expect(&format!(
        "Should have been able to read the file {:?}",
        file_path
    ));
    let asm_preprocessed = preprocess_asm(asm, additional_contracts, dictionary);
    Assembly::try_from(asm_preprocessed.to_owned())
        .unwrap()
        .compile_to_bytecode()
        .expect(&format!("Failed to compile {:?}", file_path))
}

#[test_log::test]
fn test_meta_opcode_asm() {
    run_asm_based_test(
        "src/tests/simple_tests/testdata/meta_opcode",
        &[],
        Default::default(),
    )
}
