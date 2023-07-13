use std::path::PathBuf;

use dtsfmt::test_utils::run_specs;

#[test]
fn test_specs() {
    run_specs(&PathBuf::from("./tests/specs"));
}
