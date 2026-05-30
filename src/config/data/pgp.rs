use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgpKey {
    pub public: String,
    pub private: String,
}

impl PgpKey {
    pub fn empty() -> Self {
        Self {
            public: String::new(),
            private: String::new(),
        }
    }
}
