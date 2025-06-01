use test_schema as _;

fn main() -> std::io::Result<()> {
    mimic::mimic_build!("test_schema::Canister");

    Ok(())
}
