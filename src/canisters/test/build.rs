fn main() -> std::io::Result<()> {
    // init design dependencies
    mimic_base::init();

    // build macro
    mimic::mimic_build!("test");

    Ok(())
}
