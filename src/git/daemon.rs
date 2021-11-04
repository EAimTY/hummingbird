use super::Repo;
use crate::database::DatabaseUpdate;
use tokio::sync::{mpsc, oneshot};

pub struct RepoDaemon<'a> {
    pub repo: Repo<'a>,
    pub repo_update_listener: mpsc::Receiver<oneshot::Sender<DatabaseUpdate>>,
}

impl<'a> RepoDaemon<'a> {
    pub async fn listen(mut self) {
        while let Some(responder) = self.repo_update_listener.recv().await {
            self.repo.fetch();
            let update = self.repo.get_database_update().await;
            responder.send(update).unwrap();
        }
    }
}
