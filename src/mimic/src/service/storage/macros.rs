//
// load
//

#[macro_export]
macro_rules! query_executor_load {
    () => {{ ::mimic::service::storage::LoadExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load {
    ($query:expr) => {{ ::mimic::query_executor_load!().execute($query) }};
}

#[macro_export]
macro_rules! query_executor_load_dyn {
    () => {{ ::mimic::service::storage::LoadExecutorDyn::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_load_dyn {
    ($query:expr) => {{ ::mimic::query_executor_load_dyn!().execute($query) }};
}

//
// save
//
#[macro_export]
macro_rules! query_executor_save {
    () => {{ ::mimic::service::storage::SaveExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_save {
    ($query:expr) => {{ ::mimic::query_executor_save!().execute($query) }};
}

//
// delete
//

#[macro_export]
macro_rules! query_executor_delete {
    () => {{ ::mimic::service::storage::DeleteExecutor::new(&DATA_REGISTRY, &INDEX_REGISTRY) }};
}

#[macro_export]
macro_rules! query_delete {
    ($query:expr) => {{ ::mimic::query_executor_delete!().execute($query) }};
}
