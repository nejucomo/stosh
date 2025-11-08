use std::sync::mpsc;
use std::thread::JoinHandle;

use derive_more::From;

use crate::Notification;

/// The interface to send notifications to the UI thread
#[derive(Clone, Debug, From)]
pub struct UI(mpsc::SyncSender<Notification>);

impl UI {
    /// Initialize the [UI] notification channel
    pub fn create_channel() -> (Self, mpsc::Receiver<Notification>) {
        let (sender, recv) = mpsc::sync_channel(1024);
        let ui = UI::from(sender);
        (ui, recv)
    }

    /// Spawn a thread which has its own [UI] handle and notify about any propagated errors
    pub fn spawn<F>(&self, f: F) -> JoinHandle<()>
    where
        F: FnOnce(UI) -> std::io::Result<()> + Send + 'static,
    {
        let cbself = self.clone();
        let errnotifier = self.clone();

        std::thread::spawn(move || {
            if let Err(e) = f(cbself) {
                errnotifier.notify(e);
            }
        })
    }

    /// Notify the UI controller about any notifiable event
    pub fn notify<N>(&self, notification: N)
    where
        N: Into<Notification>,
    {
        self.0.send(notification.into()).unwrap()
    }
}
