use crate::store::TicketId;

// TODO: Bring the completed types in from the various lessons
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TicketTitle;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TicketDescription;

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

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
