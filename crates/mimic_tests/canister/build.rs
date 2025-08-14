use test_design as _;

fn main() -> std::io::Result<()> {
    icu::icu_build!();
    mimic::mimic_build!("test_design::schema::Canister");

    Ok(())
}
