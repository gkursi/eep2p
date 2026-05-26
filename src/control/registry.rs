use crate::config::data::Hosts;
use crate::control::state::{Connection, FwdConnection};
use timedmap::TimedMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Registry {
    pub hosts: Hosts,
    pub connections: TimedMap<Uuid, Connection>,
    pub fwd_connections: TimedMap<Uuid, FwdConnection>,
    pub target_integrity: TimedMap<Uuid, String>,
}

impl Registry {
    pub fn new(hosts: Hosts) -> Self {
        Self {
            hosts,
            connections: TimedMap::new(),
            fwd_connections: TimedMap::new(),
            target_integrity: TimedMap::new(),
        }
    }
}
