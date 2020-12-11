use std::io::Read;

fn main() -> anyhow::Result<()> {
    let wasm = if let Some(path) = std::env::args().nth(1) {
        std::fs::read(path)?
    } else {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf)?;
        buf
    };

    print!("{}", wasm2ts::convert(&wasm)?);

    Ok(())
}
