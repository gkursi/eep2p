use super::connection::FwdConnection;
use crate::config::data::hosts::Hosts;
use timedmap::TimedMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Registry {
    pub hosts: Hosts,
    // pub requests: TimedMap<Uuid, Connection>,
    pub fwd_connections: TimedMap<Uuid, FwdConnection>,
    pub id_ownership: TimedMap<Uuid, String>,
}

impl Registry {
    pub fn new(hosts: Hosts) -> Self {
        Self {
            hosts,
            // requests: TimedMap::new(),
            fwd_connections: TimedMap::new(),
            id_ownership: TimedMap::new(),
        }
    }
}
