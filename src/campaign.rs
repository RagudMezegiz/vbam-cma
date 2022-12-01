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

mod data;
mod empire;
pub mod system;
mod unit;

use data::DataStore;
use system::System;

/// A Campaign, in addition to having the same meaning as in the VBAM rules,
/// is the control layer managing the conduct of the game itself. Every
/// campaign has a name which is used as the name of the backend database.
pub struct Campaign {
    name: String,
    data: DataStore,
    turn: i32,
}

impl Campaign {
    /// Close the data connection.
    pub async fn close(&self) {
        self.data.close().await;
    }

    /// Delete an existing campaign.
    pub fn delete(name: &str) -> Result<(), String> {
        if let Err(e) = DataStore::delete(name) {
            return Err(e.to_string());
        }
        Ok(())
    }

    /// Import systems from the specified CSV file.
    pub async fn import_systems(&mut self, file: &str) -> Result<(), String> {
        let sys = system::read_from_csv(file)?;
        if let Err(e) = self.data.add_systems(sys).await {
            return Err(e.to_string());
        }
        Ok(())
    }

    /// Return names of available campaigns.
    pub fn campaigns() -> Result<Vec<String>, String> {
        match DataStore::available_campaigns() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Delete the specified system.
    pub async fn delete_system(&self, sys: &System) -> Result<(), String> {
        match self.data.delete_system(sys).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Campaign name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Create a new campaign.
    pub async fn new(name: String) -> Result<Self, String> {
        let data = match DataStore::new(name.as_str()).await {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            name: name.to_owned(),
            data,
            turn: 0,
        })
    }

    /// Open an existing campaign.
    pub async fn open(name: &str) -> Result<Self, String> {
        let data = match DataStore::open(name).await {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };
        let turn = match data.current_turn().await {
            Ok(i) => i,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            name: name.to_owned(),
            data,
            turn,
        })
    }

    /// Return the systems in the campaign.
    pub async fn systems(&self) -> Result<Vec<System>, String> {
        match self.data.get_systems().await {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Campaign title including turn number.
    pub fn title(&self) -> String {
        format!("{} Turn {}", self.name, self.turn)
    }

    /// Update the given system, which must have a valid ID.
    pub async fn update_system(&self, sys: &System) -> Result<(), String> {
        match self.data.update_system(sys).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO Add tests
}
