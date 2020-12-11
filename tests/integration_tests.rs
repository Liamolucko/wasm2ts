const TEST_WASM: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.wasm"));
const TEST_WASM_OUTPUT: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.wasm.d.ts"));

#[test]
fn basic_wasm() {
    assert_eq!(wasm2ts::convert(TEST_WASM).unwrap(), TEST_WASM_OUTPUT);
}
