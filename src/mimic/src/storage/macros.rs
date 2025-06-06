#[macro_export]
macro_rules! query_load {
    () => {{ ::mimic::storage::LoadExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load_dyn {
    () => {{ ::mimic::storage::LoadExecutorDyn::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_save {
    () => {{ ::mimic::storage::SaveExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_delete {
    () => {{ ::mimic::storage::DeleteExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}
