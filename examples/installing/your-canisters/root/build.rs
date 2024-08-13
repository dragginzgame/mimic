fn main() -> std::io::Result<()> {
    // design deps
    mimic_base::init();
    design::init();

    // build
    mimic::mimic_build!("root");

    Ok(())
}
