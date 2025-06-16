#[macro_export]
macro_rules! query_load {
    () => {{ ::mimic::db::executor::LoadExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load_dyn {
    () => {{ ::mimic::db::executor::LoadExecutorDyn::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_save {
    () => {{ ::mimic::db::executor::SaveExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_delete {
    () => {{ ::mimic::db::executor::DeleteExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}
