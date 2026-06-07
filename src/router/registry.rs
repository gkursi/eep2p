use std::collections::HashMap;

use timedmap::TimedMap;
use uuid::Uuid;

use super::connection::FwdConnection;
use crate::config::data::hosts::Hosts;
use crate::net::state::Channel;

#[derive(Debug)]
pub struct Registry {
    pub hosts: Hosts,
    // pub requests: TimedMap<Uuid, Connection>,
    pub fwd_connections: TimedMap<Uuid, FwdConnection>,
    pub connections: HashMap<String, Channel>,
}

impl Registry {
    pub fn new(hosts: Hosts) -> Self {
        Self {
            hosts,
            // requests: TimedMap::new(),
            fwd_connections: TimedMap::new(),
            connections: HashMap::new(),
        }
    }
}
