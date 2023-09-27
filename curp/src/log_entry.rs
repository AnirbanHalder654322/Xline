use std::sync::Arc;

use curp_external_api::{
    cmd::{Command, ProposeId},
    LogIndex,
};
use serde::{Deserialize, Serialize};

use crate::rpc::ConfChangeEntry;

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LogEntry<C> {
    /// Term
    pub(crate) term: u64,
    /// Index
    pub(crate) index: LogIndex,
    /// Entry data
    pub(crate) entry_data: EntryData<C>,
}

/// Entry data of a `LogEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum EntryData<C> {
    /// `Command` entry
    Command(Arc<C>),
    /// `ConfChange` entry
    ConfChange(Box<ConfChangeEntry>), // Box to fix variant_size_differences
    /// `Shutdown` entry
    Shutdown,
}

impl<C> From<ConfChangeEntry> for EntryData<C> {
    fn from(conf_change: ConfChangeEntry) -> Self {
        EntryData::ConfChange(Box::new(conf_change))
    }
}

impl<C> From<Arc<C>> for EntryData<C> {
    fn from(cmd: Arc<C>) -> Self {
        EntryData::Command(cmd)
    }
}

impl<C> LogEntry<C>
where
    C: Command,
{
    /// Create a new `LogEntry`
    pub(super) fn new(index: LogIndex, term: u64, entry_data: impl Into<EntryData<C>>) -> Self {
        Self {
            term,
            index,
            entry_data: entry_data.into(),
        }
    }

    /// Get the id of the entry
    pub(super) fn id(&self) -> &ProposeId {
        match self.entry_data {
            EntryData::Command(ref cmd) => cmd.id(),
            EntryData::ConfChange(ref e) => e.id(),
            EntryData::Shutdown => {
                unreachable!(
                    "LogEntry::id() should not be called on {:?} entry",
                    self.entry_data
                );
            }
        }
    }
}
