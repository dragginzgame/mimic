fn main() -> std::io::Result<()> {
    // init design dependencies
    mimic::init();
    test_schema::init();

    mimic_build::mimic_build!("test_schema::Canister");

    Ok(())
}
