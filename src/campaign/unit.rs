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

//! Interface to all unit types: ships, ground, stations, etc.

use sqlx::Error;
use sqlx::SqlitePool;

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct GroundType {
    id: i64,
    name: String,
    abbr: String,
    cost: i32,
    atk: i32,
    def: i32,
}

impl GroundType {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS ground_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            abbr TEXT,
            cost INTEGER,
            atk INTEGER,
            def INTEGER)").execute(pool).await?;

        sqlx::query("INSERT INTO ground_types (name, abbr, cost, atk, def)
            VALUES ('Militia', 'MIL', 2, 4, 4),
                ('Light Infantry', 'LI', 3, 4, 4),
                ('Mobile Infantry', 'MI', 4, 4, 8),
                ('Light Armor', 'LA', 4, 8, 4),
                ('Mech Infantry', 'MECH', 8, 8, 8),
                ('Marines', 'MAR', 6, 4, 8)").execute(pool).await?;

        Ok(())
    }
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct ShipType {
    id: i64,
    class: String,
    hull: String,
    cost: i32,
    cr: i32,
    atk: i32,
    def: i32,
    cap: i32,
}

impl ShipType {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS ship_types(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            class TEXT,
            hull TEXT,
            cost INTEGER,
            cr INTEGER,
            atk INTEGER,
            def INTEGER,
            cap INTEGER DEFAULT 0)").execute(pool).await?;

        // TODO Insert civilian units? Or require import of everything?

        Ok(())
    }
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct Ship {
    id: i64,
    stype: i64,
    fleet: i64,
    crip: bool,
    moth: bool,
}

impl Ship {
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS ships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stype INTEGER REFERENCES ship_types (id),
            fleet INTEGER REFERENCES fleets (id),
            crip INTEGER DEFAULT 0,
            moth INTEGER DEFAULT 0)").execute(pool).await?;

        Ok(())
    }
}

/// Create unit tables with schemas corresponding to the options.
pub async fn create_tables(pool: &SqlitePool /* TODO add options */) -> Result<(), Error> {
    // Default to VBAM 3 pre-populated ground units table.
    GroundType::create_table(pool).await?;

    // Default to VBAM 3 ship and ship types.
    ShipType::create_table(pool).await?;
    Ship::create_table(pool).await
}
