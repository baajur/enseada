use std::sync::Arc;

use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::FutureExt;

use enseada::couchdb::db::Database;
use enseada::couchdb::repository::{Entity, Repository};
use enseada::storage::Provider;
use events::EventHandler;

use crate::digest::Digest;
use crate::entity::Blob;
use crate::error::{Error, ErrorCode};
use crate::events::RepoDeleted;
use crate::{storage, Result};

#[derive(Debug)]
pub struct BlobService {
    db: Arc<Database>,
    store: Arc<Provider>,
}

impl BlobService {
    pub fn new(db: Arc<Database>, store: Arc<Provider>) -> Self {
        Self { db, store }
    }

    pub async fn fetch_content(&self, digest: &Digest) -> Result<Vec<u8>> {
        let storage_key = storage::blob_key(digest);
        let blob = self.store.get_blob(&storage_key).await?;
        match blob {
            Some(blob) => Ok(blob.content().clone()),
            None => Err(Error::from(ErrorCode::BlobUnknown)),
        }
    }

    pub async fn delete_blob(&self, blob: &Blob) -> Result<()> {
        let storage_key = storage::blob_key(blob.digest());
        self.delete(blob).await?;
        self.store.delete_blob(&storage_key).await?;
        Ok(())
    }
}

impl Repository<Blob> for BlobService {
    fn db(&self) -> &Database {
        self.db.as_ref()
    }
}

#[async_trait]
impl EventHandler<RepoDeleted> for BlobService {
    async fn handle(&self, event: &RepoDeleted) {
        let image = format!("{}/{}", &event.group, &event.name);
        if let Err(err) = self
            .delete_all(serde_json::json!({
              "image": image,
            }))
            .await
        {
            log::error!("{}", err);
        }
    }
}
