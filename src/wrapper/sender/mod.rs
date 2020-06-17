pub trait Sender<T> {
    type Error;
    fn send(&self, msg: T) -> Result<(), Self::Error>;
}

use std::sync::mpsc;

impl<T> Sender<T> for mpsc::Sender<T> {
    type Error = mpsc::SendError<T>;

    fn send(&self, msg: T) -> Result<(), Self::Error> {
        mpsc::Sender::send(self, msg)
    }
}
