pub mod connection;
pub mod packet;
pub mod state;
pub mod util;

// connection opening/closing is handled exclusively by the controller.
// to open a connection, all the parameters that would be passed in to handle are passed to a message.
// connections aren't closed immediately.
// a reference to each open connections event channel is kept in a hashmap (address to channel).
// a cleanup event is needed for controller, both to manage connections and to remove stale
// blacklists from registry (timedmap doesnt auto remove expired entries)
