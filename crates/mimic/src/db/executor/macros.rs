#[macro_export]
macro_rules! mimic_query {
    () => {{ ::mimic::db::executor::Executor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}
