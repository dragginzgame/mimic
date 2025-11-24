use mimic_test_design as _;

fn main() -> std::io::Result<()> {
    mimic::mimic_build!("mimic_test_design::schema::Canister");

    Ok(())
}
