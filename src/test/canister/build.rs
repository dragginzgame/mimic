fn main() -> std::io::Result<()> {
    // init design dependencies
    mimic::init();
    test_schema::init();

    Ok(())
}
