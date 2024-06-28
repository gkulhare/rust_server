use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Share{
    pub id: u64,
    pub share: Bytes,
} 

#[derive(Deserialize)]
pub struct CreateShare{
    pub share: Bytes,
}

#[derive(Clone)]
pub struct ModelController{
    Db: Arc<Mutex<Vec<Option<Share>>>>,
}

// Constructor
impl ModelController{
    pub async fn new() -> Result<Self> {
        Ok( Self {
            Db: Arc::default(),
        })
    }
}

// CRUD
impl ModelController{
    pub async fn create_share(&self, share_c: CreateShare) -> Result<Share> {
        let mut db = self.Db.lock().unwrap();

        let id = db.len() as u64;
        let share = Share{
            id,
            share: share_c.share,
        };
        db.push(Some(share.clone()));

        Ok(share)
    }

    pub async fn list_shares(&self) ->  Result<Vec<Share>> {
        let db = self.Db.lock().unwrap();

        let shares = db.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_share(&self, id: u64) -> Result<Share> {
        let mut db = self.Db.lock().unwrap();

        let share = db.get_mut(id as usize).and_then(|t| t.take());

        share.ok_or(Error::ShareDeleteFailNotFound {id})
    }
}