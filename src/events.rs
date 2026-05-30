use crate::Client;

pub struct EventsAPI {
    inner: Client
}

impl EventsAPI {
    pub(crate) fn new(client: Client) -> Self {
        Self { inner: client }
    }
}