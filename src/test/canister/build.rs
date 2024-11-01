fn main() -> std::io::Result<()> {
    // init design dependencies
    orm::base::init();
    test_schema::init();

    mimic::mimic_build!("test");

    Ok(())
}
