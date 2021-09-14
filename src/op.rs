use tokio::sync::oneshot;

pub enum Op {
    GetPost {
        title: String,
        channel_sender: oneshot::Sender<String>,
    },
    GetPage {
        title: String,
        channel_sender: oneshot::Sender<String>,
    },
}
