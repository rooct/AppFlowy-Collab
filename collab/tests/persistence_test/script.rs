use collab::plugin_impl::disk::CollabDiskPlugin;
use collab::preclude::*;
use lib0::any::Any;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

pub enum Script {
    CreateDocument {
        id: String,
    },
    OpenDocument {
        id: String,
    },
    CloseDocument {
        id: String,
    },
    DeleteDocument {
        id: String,
    },
    InsertText {
        id: String,
        key: String,
        value: Any,
    },
    GetText {
        id: String,
        key: String,
        expected: Option<Any>,
    },
    AssertNumOfUpdates {
        id: String,
        expected: usize,
    },
    AssertNumOfDocuments {
        expected: usize,
    },
}

pub struct CollabPersistenceTest {
    collabs: HashMap<String, Collab>,
    disk_plugin: CollabDiskPlugin,
    #[allow(dead_code)]
    cleaner: Cleaner,
    pub db_path: PathBuf,
}

impl CollabPersistenceTest {
    pub fn new() -> Self {
        let tempdir = TempDir::new().unwrap();
        let path = tempdir.into_path();
        let disk_plugin = CollabDiskPlugin::new(path.clone()).unwrap();
        let cleaner = Cleaner::new(path.clone());
        Self {
            collabs: HashMap::default(),
            disk_plugin,
            cleaner,
            db_path: path,
        }
    }

    pub fn run_scripts(&mut self, scripts: Vec<Script>) {
        for script in scripts {
            self.run_script(script);
        }
    }

    pub fn run_script(&mut self, script: Script) {
        match script {
            Script::CreateDocument { id } => {
                let collab = CollabBuilder::new(1, &id)
                    .with_plugin(self.disk_plugin.clone())
                    .build();
                self.collabs.insert(id, collab);
            }
            Script::CloseDocument { id } => {
                self.collabs.remove(&id);
            }
            Script::OpenDocument { id } => {
                let collab = CollabBuilder::new(1, &id)
                    .with_plugin(self.disk_plugin.clone())
                    .build();
                self.collabs.insert(id, collab);
            }
            Script::DeleteDocument { id } => {
                self.disk_plugin.doc().delete_doc(&id).unwrap();
            }
            Script::InsertText { id, key, value } => {
                self.collabs.get(&id).as_ref().unwrap().insert(&key, value);
            }
            Script::GetText { id, key, expected } => {
                let collab = self.collabs.get(&id).unwrap();
                let txn = collab.transact();
                let text = collab
                    .get(&key)
                    .map(|value| value.to_string(&txn))
                    .map(|value| Any::String(value.into_boxed_str()));
                assert_eq!(text, expected)
            }
            Script::AssertNumOfUpdates { id, expected } => {
                let updates = self.disk_plugin.doc().get_updates(&id).unwrap();
                assert_eq!(updates.len(), expected)
            }
            Script::AssertNumOfDocuments { expected } => {
                let docs = self.disk_plugin.doc().get_all_docs().unwrap();
                assert_eq!(docs.count(), expected);
            }
        }
    }
}

struct Cleaner(PathBuf);

impl Cleaner {
    fn new(dir: PathBuf) -> Self {
        Cleaner(dir)
    }

    fn cleanup(dir: &PathBuf) {
        let _ = std::fs::remove_dir_all(dir);
    }
}

impl Drop for Cleaner {
    fn drop(&mut self) {
        Self::cleanup(&self.0)
    }
}
