#[macro_export]
macro_rules! query_load {
    () => {{ ::mimic::service::storage::LoadExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load_dyn {
    () => {{ ::mimic::service::storage::LoadExecutorDyn::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_save {
    () => {{ ::mimic::service::storage::SaveExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_delete {
    () => {{ ::mimic::service::storage::DeleteExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}
