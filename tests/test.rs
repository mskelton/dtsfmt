use std::path::PathBuf;

use dtsfmt::test_utils::run_specs;

#[test]
fn test_specs() {
    run_specs(&PathBuf::from("./tests/specs"));
}

#[test]
fn test_array_wrapping_config() {
    use dtsfmt::config::Config;
    use dtsfmt::printer::print;

    let source = "/dts-v1/;\n\n/ {\n  big = <0x1 0x2 0x3 0x4 0x5>;\n};\n";

    let default_config = Config::default();
    assert_eq!(
        print(&source.to_owned(), &default_config),
        "/dts-v1/;\n\n/ {\n  big = <0x1 0x2 0x3 0x4 0x5>;\n};\n"
    );

    let wrapped_config = Config::builder()
        .array_wrap_threshold(4)
        .array_cells_per_line(2)
        .build();
    assert_eq!(
        print(&source.to_owned(), &wrapped_config),
        "/dts-v1/;\n\n/ {\n  big = <\n    0x1 0x2\n    0x3 0x4\n    0x5\n  >;\n};\n"
    );
}
