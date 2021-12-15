use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

/// Tree Structure
#[derive(Serialize, Deserialize)]
pub struct StreeSerde {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String, String)>,
}

impl StreeSerde {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn dump(&self, fpath: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(fpath)?;
        file.write(self.serialize().as_bytes())
    }
}
