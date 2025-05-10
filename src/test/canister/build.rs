use {mimic as _, test_schema as _};

fn main() -> std::io::Result<()> {
    mimic_build::mimic_build!("test_schema::Canister");

    Ok(())
}
