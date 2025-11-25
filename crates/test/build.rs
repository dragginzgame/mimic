use test_design as _;

fn main() -> std::io::Result<()> {
    mimic::mimic_build!("test_design::schema::Canister");

    Ok(())
}
