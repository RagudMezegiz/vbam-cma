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

//! Interface to star systems.

use sqlx::Error;
use sqlx::SqlitePool;

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct System {
    id: i64,
    name: String,
    ptype: String,
    raw: i32,
    cap: i32,
    pop: i32,
    mor: i32,
    ind: i32,
    dev: i32,
    fails: i32,
    owner: i64,
}

impl System {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS system (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            ptype TEXT,
            raw INTEGER,
            cap INTEGER,
            pop INTEGER,
            mor INTEGER,
            ind INTEGER,
            dev INTEGER DEFAULT 0,
            fails INTEGER DEFAULT 0,
            owner INTEGER REFERENCES empire (id))").execute(pool).await?;
        Ok(())
    }
}

/// Create the Systems table with schema corresponding to the options.
pub async fn create_table(pool: &SqlitePool /* TODO add options */) -> Result<(), Error> {
    // Default to playtest VBAM3 schema
    System::create_table(pool).await
}

#[cfg(test)]
mod tests {
    // TODO Add tests
}
