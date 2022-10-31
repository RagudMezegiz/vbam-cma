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

use sqlx::SqlitePool;
use std::{fs, io, path};

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
        let dbpath = database_path(&name)?;
        if dbpath.exists() {
            // This database already exists, so can't create a new campaign
            // the same name.
            return Err("Campaign already exists".to_string());
        }

        // Create and connect to the database.
        let url = format!("sqlite://{}?mode=rwc", dbpath.to_str().unwrap());
        let pool = match SqlitePool::connect(url.as_str()).await {
            Ok(p) => p,
            Err(e) => return Err(e.to_string())
        };

        // TODO Use options to create initial database tables

        Ok(Self { name, pool })
    }

    /// Open an existing campaign.
    pub async fn open(name: &String) -> Result<Self, String> {
        let dbpath = database_path(name)?;

        // Connect to the database.
        let url = format!("sqlite://{}", dbpath.to_str().unwrap());
        let pool = match SqlitePool::connect(url.as_str()).await {
            Ok(p) => p,
            Err(e) => return Err(e.to_string())
        };

        Ok(Self { name: name.clone(), pool })
    }

    /// Delete an existing campaign.
    pub fn delete(name: &String) -> Result<(), String> {
        let dbpath = database_path(name)?;
        match fs::remove_file(dbpath) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Campaign name.
    pub fn name(&self) -> &String {
        &self.name
    }
}

fn data_folder() -> Result<path::PathBuf, String> {
    // Put databases in the user's data directory...
    let mut dbpath = if let Some(p) = dirs::data_dir() {
        p
    } else {
        path::PathBuf::new()
    };
    // ... under the program name.
    dbpath.push("vbamcma");

    // Create folder if it doesn't exist.
    if !dbpath.exists() {
        match fs::create_dir_all(&dbpath) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(dbpath)
}

fn database_path(name: &str) -> Result<path::PathBuf, String> {
    // Create SQLite file name by converting spaces in the campaign name
    // to underscores and adding the '.db' extension.
    let dbname = name.replace(' ', "_") + ".db";

    let mut dbpath = data_folder()?;
    dbpath.push(dbname);

    Ok(dbpath)
}

/// List all available campaigns.
pub fn list() -> io::Result<Vec<String>> {
    let folder = match data_folder() {
        Ok(p) => p,
        Err(_) => return Err(io::Error::last_os_error()),
    };
    let names = fs::read_dir(folder)?
        .filter(|f| {
            match f {
                Ok(f) => match f.path().extension() {
                    Some(e) => e == "db",
                    _ => false,
                },
                _ => false,
            }})
        .map(|f|
            match f {
                Ok(f) => match f.path().file_stem() {
                    Some(f) => match f.to_str() {
                        Some(s) => s.replace('_', " "),
                        _ => String::new(),
                    },
                    _ => String::new(),
                },
                _ => String::new(),
            })
        .collect();
    Ok(names)
}
