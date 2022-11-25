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

//! Data storage layer.

use sqlx::{Row, SqlitePool};
use std::{error, fmt, fs, io, num, path};

use super::system::System;

type DataResult<T> = Result<T, DataError>;

/// Data storage layer Error type.
#[derive(Debug)]
pub enum DataError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Sqlx(sqlx::Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Io(e) => e.to_string(),
                Self::Parse(e) => e.to_string(),
                Self::Sqlx(e) => e.to_string(),
            }
        )
    }
}

impl error::Error for DataError {}

impl From<io::Error> for DataError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<num::ParseIntError> for DataError {
    fn from(e: num::ParseIntError) -> Self {
        Self::Parse(e)
    }
}

impl From<sqlx::Error> for DataError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

/// Persistent storage for a campaign's data.
pub struct DataStore {
    pool: SqlitePool,
}

impl DataStore {
    /// Add systems to the store.
    pub async fn add_systems(&self, systems: Vec<System>) -> DataResult<()> {
        for s in systems {
            self.insert_system(s).await?
        }
        Ok(())
    }

    /// Return list of available campaigns.
    pub fn available_campaigns() -> DataResult<Vec<String>> {
        let folder = Self::folder()?;
        let rd = fs::read_dir(folder)?;
        let names = rd
            .filter(|f| match f {
                Ok(f) => match f.path().extension() {
                    Some(e) => e == "db",
                    _ => false,
                },
                _ => false,
            })
            .map(|f| match f {
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

    /// Close the underlying storage.
    pub async fn close(&self) {
        self.pool.close().await
    }

    /// Return the current turn number.
    pub async fn current_turn(&self) -> DataResult<i32> {
        let r = sqlx::query("SELECT value FROM control WHERE key = 'turn'")
            .fetch_one(&self.pool)
            .await?;
        let val: String = r.get("value");
        let turn = val.parse::<i32>()?;
        Ok(turn)
    }

    /// Delete a persistent store by name.
    pub fn delete(name: &str) -> DataResult<()> {
        let dbpath = Self::path(name)?;
        fs::remove_file(dbpath)?;
        Ok(())
    }

    /// Return the name for the empire ID.
    pub async fn get_empire_name(&self, id: i64) -> DataResult<String> {
        let n = sqlx::query("SELECT name FROM empires WHERE id=?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(n.get(0))
    }

    /// Return a system by name.
    #[allow(unused)]
    pub async fn get_system_by_name(&self, name: &str) -> DataResult<System> {
        let mut sys: System = sqlx::query_as("SELECT * FROM systems WHERE NAME = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        sys.owner_name = match sys.owner {
            0 => "None".to_string(),
            n => self.get_empire_name(n).await?,
        };
        Ok(sys)
    }

    /// Return the systems from the store.
    pub async fn get_systems(&self) -> DataResult<Vec<System>> {
        let v: Vec<System> = sqlx::query_as("SELECT * FROM systems")
            .fetch_all(&self.pool)
            .await?;
        let mut res = Vec::new();
        for mut s in v {
            s.owner_name = match s.owner {
                0 => "None".to_string(),
                n => self.get_empire_name(n).await?,
            };
            res.push(s)
        }
        Ok(res)
    }

    /// Create a new data store using the specified name.
    pub async fn new(name: &str) -> DataResult<Self> {
        let dbpath = Self::path(name)?;
        if dbpath.exists() {
            // This database already exists, so can't create a new campaign
            // with the same name.
            return Err(DataError::Io(io::Error::from(io::ErrorKind::AlreadyExists)));
        }

        // Create and connect to the database.
        let url = format!("sqlite://{}?mode=rwc", dbpath.to_str().unwrap());
        let pool = SqlitePool::connect(url.as_str()).await?;

        Self::create_tables(&pool).await?;
        Ok(Self { pool })
    }

    /// Open an existing data store.
    pub async fn open(name: &str) -> DataResult<Self> {
        let dbpath = Self::path(name)?;

        // Connect to the database.
        let url = format!("sqlite://{}", dbpath.to_str().unwrap());
        let pool = SqlitePool::connect(url.as_str()).await?;

        Ok(Self { pool })
    }

    async fn create_controls_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS control (
            key TEXT PRIMARY KEY,
            value TEXT)",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO control VALUES
            ('turn', '0')",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_empires_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS empires (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            treasury INTEGER DEFAULT 0,
            tech INTEGER DEFAULT 0)",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_fleets_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS fleets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            owner INTEGER REFERENCES empires (id),
            location INTEGER REFERENCES systems (id))",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_ground_types_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ground_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            abbr TEXT,
            cost INTEGER,
            atk INTEGER,
            def INTEGER)",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO ground_types
            (name, abbr, cost, atk, def)
            VALUES
            ('Militia', 'MIL', 2, 4, 4),
            ('Light Infantry', 'LI', 3, 4, 4),
            ('Mobile Infantry', 'MI', 4, 4, 8),
            ('Light Armor', 'LA', 4, 8, 4),
            ('Mech Infantry', 'MECH', 8, 8, 8),
            ('Marines', 'MAR', 6, 4, 8)",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_ground_units_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ground_units (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            gtype INTEGER REFERENCES ground_types (id),
            loc INTEGER REFERENCES systems (id))",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_ship_types_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ship_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            class TEXT,
            hull TEXT,
            cost INTEGER,
            cr INTEGER,
            atk INTEGER,
            def INTEGER,
            cap INTEGER DEFAULT 0,
            empire INTEGER REFERENCES empires (id))",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_ships_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stype INTEGER REFERENCES ship_types (id),
            fleet INTEGER REFERENCES fleets (id),
            crip INTEGER DEFAULT 0,
            moth INTEGER DEFAULT 0)",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_systems_table(pool: &SqlitePool) -> DataResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS systems (
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
            owner INTEGER REFERENCES empires (id))",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_tables(pool: &SqlitePool) -> DataResult<()> {
        Self::create_controls_table(pool).await?;
        Self::create_empires_table(pool).await?;
        Self::create_fleets_table(pool).await?;
        Self::create_ground_types_table(pool).await?;
        Self::create_ground_units_table(pool).await?;
        Self::create_ship_types_table(pool).await?;
        Self::create_ships_table(pool).await?;
        Self::create_systems_table(pool).await
    }

    fn folder() -> DataResult<path::PathBuf> {
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
            fs::create_dir_all(&dbpath)?
        }

        Ok(dbpath)
    }

    async fn insert_system(&self, sys: System) -> DataResult<()> {
        sqlx::query(
            "INSERT INTO systems (name, ptype, raw, cap, pop, mor, ind)
            VALUES(?,?,?,?,?,?,?)",
        )
        .bind(sys.name.as_str())
        .bind(sys.ptype.as_str())
        .bind(sys.raw)
        .bind(sys.cap)
        .bind(sys.pop)
        .bind(sys.mor)
        .bind(sys.ind)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    fn path(name: &str) -> DataResult<path::PathBuf> {
        // Create SQLite file name by converting spaces in the campaign name
        // to underscores and adding the '.db' extension.
        let dbname = name.replace(' ', "_") + ".db";

        let mut dbpath = Self::folder()?;
        dbpath.push(dbname);

        Ok(dbpath)
    }
}

#[cfg(test)]
mod tests {
    use super::DataStore;
    use crate::campaign::system::tests::systems;

    async fn init_data() -> DataStore {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        DataStore::create_tables(&pool).await.unwrap();
        DataStore { pool }
    }

    #[tokio::test]
    async fn add_systems() {
        let instance = init_data().await;
        instance.add_systems(systems()).await.unwrap();
        for exp in systems() {
            let act = instance
                .get_system_by_name(exp.name.as_str())
                .await
                .unwrap();
            assert_eq!(exp.name, act.name);
            assert_eq!(exp.ptype, act.ptype);
            assert_eq!(exp.raw, act.raw);
            assert_eq!(exp.cap, act.cap);
            assert_eq!(exp.pop, act.pop);
            assert_eq!(exp.mor, act.mor);
            assert_eq!(exp.ind, act.ind);
            assert_eq!(exp.dev, act.dev);
            assert_eq!(exp.fails, act.fails);
        }
    }

    #[tokio::test]
    async fn current_turn() {
        let instance = init_data().await;
        assert_eq!(0, instance.current_turn().await.unwrap());
    }
}
