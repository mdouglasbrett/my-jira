use crate::store::TicketId;

use std::convert::TryFrom;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TicketTitle(String);

fn check_title(title: &str) -> Result<String, TicketTitleError> {
    if title.is_empty() {
        return Err(TicketTitleError::Empty);
    }
    if title.len() > 50 {
        return Err(TicketTitleError::TooLong);
    }
    Ok(title.to_string())
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let title = check_title(value)?;
        Ok(Self(title))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let title = check_title(&value)?;
        Ok(Self(title))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than 500 bytes")]
    TooLong,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TicketDescription(String);

fn check_description(desc: &str) -> Result<String, TicketDescriptionError> {
    if desc.is_empty() {
        return Err(TicketDescriptionError::Empty);
    }
    if desc.len() > 500 {
        return Err(TicketDescriptionError::TooLong);
    }
    Ok(desc.to_string())
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let description = check_description(value)?;
        Ok(Self(description))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let description = check_description(&value)?;
        Ok(Self(description))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketPatch {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}


#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
