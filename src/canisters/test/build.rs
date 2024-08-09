fn main() -> std::io::Result<()> {
    // init design dependencies
    mimic_base::init();

    mimic::mimic_build!("test");

    Ok(())
}
