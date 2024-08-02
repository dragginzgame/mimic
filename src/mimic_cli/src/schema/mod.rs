use orm_schema::build::schema;

// process
pub fn process() {
    let output = serde_json::to_string(&*schema()).unwrap();

    println!("{output}");
}
