use crate::net::state::Channel;

#[derive(Debug, Clone)]
pub enum Command {
    TryOpenConnection { target: String },

    AddConnection { origin: String, channel: Channel },
}
