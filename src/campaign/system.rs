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

use std::io;

use sqlx::Error;
use sqlx::SqlitePool;

use super::empire;

#[allow(unused)]
#[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq)]
pub struct System {
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
    // Convert to string as a row of tab-separated fields.
    pub async fn as_row(&self, pool: &SqlitePool) -> String {
        format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.name, self.ptype, self.raw, self.cap, self.pop, self.mor,
            self.ind, self.dev, self.fails, self.get_owner_name(pool).await)
    }

    // Create the systems table.
    async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS systems (
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
            owner INTEGER REFERENCES empires (id))").execute(pool).await?;
        Ok(())
    }

    // Create a new system from a CSV record
    fn from_csv(rcd: csv::StringRecord) -> Result<System, csv::Error> {
        let err = csv::Error::from(io::Error::from(io::ErrorKind::InvalidInput));
        let name = match rcd.get(0) {
            Some(n) => n,
            None => return Err(err),
        };
        let ptype = match rcd.get(1) {
            Some(p) => p,
            None => return Err(err),
        };
        let raw = match rcd.get(2) {
            Some(r) => match r.parse() {
                Ok(r) => r,
                Err(_) => return Err(err),
            },
            None => return Err(err),
        };
        let cap = match rcd.get(3) {
            Some(c) => match c.parse() {
                Ok(c) => c,
                Err(_) => return Err(err),
            },
            None => return Err(err),
        };
        let pop = match rcd.get(4) {
            Some(p) => match p.parse() {
                Ok(p) => p,
                Err(_) => return Err(err),
            },
            None => return Err(err),
        };
        let mor = match rcd.get(5) {
            Some(m) => match m.parse() {
                Ok(m) => m,
                Err(_) => return Err(err),
            },
            None => return Err(err),
        };
        let ind = match rcd.get(6) {
            Some(i) => match i.parse() {
                Ok(i) => i,
                Err(_) => return Err(err),
            },
            None => return Err(err),
        };

        Ok(Self::new(name, ptype, raw, cap, pop, mor, ind))
    }

    // Return the owning empire's name, or "None" if unowned.
    async fn get_owner_name(&self, pool: &SqlitePool) -> String {
        match empire::by_id(pool, self.owner).await {
            Some(e) => e.name(),
            None => "None".to_string(),
        }
    }

    // Import systems from a CSV text stream and write to database.
    async fn import<R>(mut rdr: csv::Reader<R>, pool: &SqlitePool) -> Result<(), String>
        where R: io::Read {
        for result in rdr.records() {
            match result {
                Ok(rcd) => {
                    if let Ok(sys) = Self::from_csv(rcd) {
                        if let Err(e) = sys.insert(pool).await {
                            return Err(e.to_string())
                        }
                    }
                },
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(())
    }

    // Insert this system into the database.
    async fn insert(&self, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("INSERT INTO systems (name, ptype, raw, cap, pop, mor, ind)
            VALUES(?,?,?,?,?,?,?)")
            .bind(self.name.as_str())
            .bind(self.ptype.as_str())
            .bind(self.raw)
            .bind(self.cap)
            .bind(self.pop)
            .bind(self.mor)
            .bind(self.ind)
            .execute(pool).await?;
        Ok(())
    }

    // Create a new system.
    fn new(name: &str, ptype: &str, raw: i32, cap: i32, pop: i32,
        mor: i32, ind: i32) -> System {
        Self { id: 0, name: name.to_string(), ptype: ptype.to_string(),
            raw, cap, pop, mor, ind, dev: 0, fails: 0, owner: 0 }
    }

    // Select all systems from the database.
    async fn select_all(pool: &SqlitePool) -> Result<Vec<System>, Error> {
        sqlx::query_as("SELECT * FROM systems")
            .fetch_all(pool).await
    }

    // Select a system from the database by name.
    async fn select_by_name(name: &str, pool: &SqlitePool) -> Result<System, Error> {
        sqlx::query_as("SELECT * FROM systems WHERE NAME = ?")
            .bind(name)
            .fetch_one(pool).await
    }
}

/// Create the Systems table with schema corresponding to the options.
pub async fn create_table(pool: &SqlitePool /* TODO add options */) -> Result<(), Error> {
    // Default to playtest VBAM3 schema
    System::create_table(pool).await
}

/// Return all systems from the table.
pub async fn get_all(pool: &SqlitePool) -> Result<Vec<System>, Error> {
    System::select_all(pool).await
}

/// Import systems from a CSV file.
pub async fn import(pool: &SqlitePool, file: &str) -> Result<(), String> {
    let r = match csv::Reader::from_path(file) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    System::import(r, pool).await
}

#[cfg(test)]
mod tests {
    use crate::campaign::Campaign;
    use crate::campaign::system::System;
    use csv::Reader;
    use sqlx::SqlitePool;

    const SYSTEM_IMPORT: &[u8] = "NAME,TYPE,RAW,CAP,POP,MOR,IND\n\
        Senor Prime,HW,5,12,10,8,10\n\
        Vadurrinia,Adaptable,3,8,4,3,3\n\
        Zev'rch,Barren,2,6,3,2,2\n\
        Tibron,Barren,4,6,3,2,3\n".as_bytes();

    fn systems() -> Vec<System> {
        let mut sys = Vec::new();
        sys.push(System::new("Senor Prime", "HW",
            5, 12, 10, 8, 10));
        sys.push(System::new("Vadurrinia", "Adaptable",
            3, 8, 4, 3, 3));
        sys.push(System::new("Zev'rch", "Barren",
            2, 6, 3, 2, 2));
        sys.push(System::new("Tibron", "Barren",
            4, 6, 3, 2, 3));
        sys
    }

    #[test]
    fn deserialize() {
        let exp = systems();
        let mut rdr = Reader::from_reader(SYSTEM_IMPORT);
        let mut count = 0;
        for result in rdr.records() {
            let record = result.unwrap();
            let val = System::from_csv(record).unwrap();
            assert!(exp.contains(&val));
            count += 1;
        }
        assert_eq!(count, exp.len());
    }

    #[tokio::test]
    async fn import() {
        let rdr = Reader::from_reader(SYSTEM_IMPORT);
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        Campaign::create_tables(&pool).await.unwrap();
        System::import(rdr, &pool).await.unwrap();
        for exp in systems() {
            let act = System::select_by_name(exp.name.as_str(), &pool).await.unwrap();
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
}
