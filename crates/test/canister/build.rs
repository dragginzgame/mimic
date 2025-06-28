use test_design as _;

fn main() -> std::io::Result<()> {
    mimic_build::build!("test_design::schema::Canister");

    Ok(())
}
