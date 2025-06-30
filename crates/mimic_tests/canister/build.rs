use test_design as _;

fn main() -> std::io::Result<()> {
    mimic::build!("test_design::schema::Canister");

    Ok(())
}
