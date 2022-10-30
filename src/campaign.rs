// Copyright 2022 David Terhune
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The program interface to the back-end data and control layer.

use dirs;
use sqlx::SqlitePool;
use std::{fs, path};

/// A Campaign, in addition to having the same meaning as in the VBAM rules,
/// is the control layer managing the conduct of the game itself. Every
/// campaign has a name which is used as the name of the backend database.
pub struct Campaign {
    name: String,
    pool: SqlitePool,
}

impl Campaign {
    /// Close the data connection.
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Create a new campaign.
    pub async fn new(name: String) -> Result<Self, String> {
        // Create SQLite file name by stripping spaces from the campaign
        // name and adding the '.db' extension.
        let dbname = name.replace(" ", "") + ".db";
        let mut dbpath = if let Some(p) = dirs::data_dir() {
            p
        } else {
            path::PathBuf::new()
        };
        dbpath.push("vbamcma");
        if !dbpath.exists() {
            match fs::create_dir_all(&dbpath) {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }
        }
        dbpath.push(dbname);
        let url = format!("sqlite://{}?mode=rwc", dbpath.to_str().unwrap());
        if dbpath.exists() {
            return Err("Campaign already exists".to_string());
        }
        println!("{}", url);
        let pool = match SqlitePool::connect(url.as_str()).await {
                Ok(p) => p,
                Err(e) => return Err(e.to_string())
            };
        // TODO Use options to create initial database tables
        Ok(Self { name, pool })
    }
}
