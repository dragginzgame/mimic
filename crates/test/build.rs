use test_design as _;

fn main() -> std::io::Result<()> {
    icydb::mimic_build!("test_design::schema::Canister");

    Ok(())
}
