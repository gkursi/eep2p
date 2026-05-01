use crate::connection::ConnectionInfo;
use timedmap::TimedMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Registry {
    connections: TimedMap<Uuid, Connection>,
    fwd_connections: TimedMap<Uuid, FwdConnection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Connection {
    target: String,
    id: Uuid,
    sock: ConnectionInfo
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FwdConnection {
    origin: String,
    origin_id: Uuid,
    target: String,
}
