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

//! Interface to empires.

use sqlx::Error;
use sqlx::SqlitePool;

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct Empire {
    id: i64,
    name: String,
    tech: i32,
}

impl Empire {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS empires (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            tech INTEGER DEFAULT 0)").execute(pool).await?;

        Ok(())
    }
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct Fleet {
    id: i64,
    name: String,
    owner: i64,
    location: i64,
}

impl Fleet {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS fleets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            owner INTEGER REFERENCES empires (id),
            location INTEGER REFERENCES systems (id))").execute(pool).await?;

        Ok(())
    }
}

/// Create the Empires table with schema according to the options.
pub async fn create_table(pool: &SqlitePool /* TODO add options */) -> Result<(), Error> {
    Empire::create_table(pool).await?;
    Fleet::create_table(pool).await
}

#[cfg(test)]
mod tests {
    // TODO Add tests
}
