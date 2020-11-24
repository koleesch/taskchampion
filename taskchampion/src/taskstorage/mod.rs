use failure::Fallible;
use std::collections::HashMap;
use uuid::Uuid;

mod inmemory;
mod kv;
mod operation;

pub use self::kv::KVStorage;
pub use inmemory::InMemoryStorage;

pub use operation::Operation;

/// An in-memory representation of a task as a simple hashmap
pub type TaskMap = HashMap<String, String>;

#[cfg(test)]
fn taskmap_with(mut properties: Vec<(String, String)>) -> TaskMap {
    let mut rv = TaskMap::new();
    for (p, v) in properties.drain(..) {
        rv.insert(p, v);
    }
    rv
}

/// A TaskStorage transaction, in which storage operations are performed.
///
/// # Concurrency
///
/// Serializable consistency must be maintained.  Concurrent access is unusual
/// and some implementations may simply apply a mutex to limit access to
/// one transaction at a time.
///
/// # Commiting and Aborting
///
/// A transaction is not visible to other readers until it is committed with
/// [`crate::taskstorage::TaskStorageTxn::commit`].  Transactions are aborted if they are dropped.
/// It is safe and performant to drop transactions that did not modify any data without committing.
pub trait TaskStorageTxn {
    /// Get an (immutable) task, if it is in the storage
    fn get_task(&mut self, uuid: &Uuid) -> Fallible<Option<TaskMap>>;

    /// Create an (empty) task, only if it does not already exist.  Returns true if
    /// the task was created (did not already exist).
    fn create_task(&mut self, uuid: Uuid) -> Fallible<bool>;

    /// Set a task, overwriting any existing task.  If the task does not exist, this implicitly
    /// creates it (use `get_task` to check first, if necessary).
    fn set_task(&mut self, uuid: Uuid, task: TaskMap) -> Fallible<()>;

    /// Delete a task, if it exists.  Returns true if the task was deleted (already existed)
    fn delete_task(&mut self, uuid: &Uuid) -> Fallible<bool>;

    /// Get the uuids and bodies of all tasks in the storage, in undefined order.
    fn all_tasks(&mut self) -> Fallible<Vec<(Uuid, TaskMap)>>;

    /// Get the uuids of all tasks in the storage, in undefined order.
    fn all_task_uuids(&mut self) -> Fallible<Vec<Uuid>>;

    /// Get the current base_version for this storage -- the last version synced from the server.
    fn base_version(&mut self) -> Fallible<u64>;

    /// Set the current base_version for this storage.
    fn set_base_version(&mut self, version: u64) -> Fallible<()>;

    /// Get the current set of outstanding operations (operations that have not been sync'd to the
    /// server yet)
    fn operations(&mut self) -> Fallible<Vec<Operation>>;

    /// Add an operation to the end of the list of operations in the storage.  Note that this
    /// merely *stores* the operation; it is up to the TaskDB to apply it.
    fn add_operation(&mut self, op: Operation) -> Fallible<()>;

    /// Replace the current list of operations with a new list.
    fn set_operations(&mut self, ops: Vec<Operation>) -> Fallible<()>;

    /// Get the entire working set, with each task UUID at its appropriate (1-based) index.
    /// Element 0 is always None.
    fn get_working_set(&mut self) -> Fallible<Vec<Option<Uuid>>>;

    /// Add a task to the working set and return its (one-based) index.  This index will be one greater
    /// than the highest used index.
    fn add_to_working_set(&mut self, uuid: &Uuid) -> Fallible<usize>;

    /// Remove a task from the working set.  Other tasks' indexes are not affected.
    fn remove_from_working_set(&mut self, index: usize) -> Fallible<()>;

    /// Clear all tasks from the working set in preparation for a garbage-collection operation.
    fn clear_working_set(&mut self) -> Fallible<()>;

    /// Commit any changes made in the transaction.  It is an error to call this more than
    /// once.
    fn commit(&mut self) -> Fallible<()>;
}

/// A trait for objects able to act as task storage.  Most of the interesting behavior is in the
/// [`crate::taskstorage::TaskStorageTxn`] trait.
pub trait TaskStorage {
    /// Begin a transaction
    fn txn<'a>(&'a mut self) -> Fallible<Box<dyn TaskStorageTxn + 'a>>;
}
