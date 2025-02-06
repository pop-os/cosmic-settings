use std::error::Error as ErrorTrait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("failed to cancel upgrade")]
    Cancel(#[source] zbus::Error),
    #[error("failed to dismiss notifications")]
    Dismiss(bool, #[source] StatusError),
    #[error("failed to finalize release upgrade")]
    Finalize(#[source] zbus::Error),
    #[error("recovery upgrade failed")]
    Recovery(#[source] StatusError),
    #[error("failed to set up OS refresh")]
    Refresh(#[source] StatusError),
    #[error("failed to update system")]
    Updates(#[source] StatusError),
    #[error("failed to upgrade OS")]
    Upgrade(#[source] StatusError),
}

impl UiError {
    pub fn iter_sources(&self) -> ErrorIter<'_> {
        ErrorIter {
            current: self.source(),
        }
    }
}

#[derive(Debug, Error)]
#[error("{}", _0)]
pub struct StatusError(pub Box<str>);

impl From<zbus::Error> for StatusError {
    fn from(value: zbus::Error) -> Self {
        StatusError(value.to_string().into())
    }
}

impl From<String> for StatusError {
    fn from(value: String) -> Self {
        StatusError(value.into())
    }
}

impl From<Box<str>> for StatusError {
    fn from(value: Box<str>) -> Self {
        StatusError(value)
    }
}

pub struct ErrorIter<'a> {
    current: Option<&'a (dyn ErrorTrait + 'static)>,
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a (dyn ErrorTrait + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(ErrorTrait::source);
        current
    }
}
