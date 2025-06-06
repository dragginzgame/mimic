#[macro_export]
macro_rules! query_load {
    () => {{ ::mimic::data::executor::LoadExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load_dyn {
    () => {{ ::mimic::data::executor::LoadExecutorDyn::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_save {
    () => {{ ::mimic::data::executor::SaveExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_delete {
    () => {{ ::mimic::data::executor::DeleteExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}
