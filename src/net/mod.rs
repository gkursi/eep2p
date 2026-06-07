pub mod connection;
pub mod error;
pub mod message;
pub mod state;
pub mod task;
pub mod util;

// todo
// all newly opened and/or accepted connections are passed via the same message to the router by the client/server setup handler
//
// a reference to each open connections event channel is kept in a hashmap (address to channel).
// a cleanup event is needed for controller, both to manage connections and to remove stale
// blacklists from registry (timedmap doesnt auto remove expired entries)
