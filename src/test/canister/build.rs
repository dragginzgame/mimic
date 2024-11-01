fn main() -> std::io::Result<()> {
    // init design dependencies
    mimic::init();
    test_schema::init();

    mimic::mimic_build!("test");

    Ok(())
}
