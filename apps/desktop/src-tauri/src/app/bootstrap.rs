use super::runtime::IanRuntime;
use crate::storage::StorageService;

pub fn bootstrap_runtime() -> IanRuntime {
    let storage = StorageService::initialize().unwrap_or_else(|_| StorageService::in_memory());
    IanRuntime::new(storage)
}
