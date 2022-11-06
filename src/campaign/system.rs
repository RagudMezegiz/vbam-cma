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
#[derive(sqlx::FromRow, Debug, PartialEq)]
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
    // Create the systems table.
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

    // Create a new system.
    fn new(name: &str, ptype: &str, raw: i32, cap: i32, pop: i32,
        mor: i32, ind: i32) -> System {
        Self { id: 0, name: name.to_string(), ptype: ptype.to_string(),
            raw, cap, pop, mor, ind, dev: 0, fails: 0, owner: 0 }
    }
}

/// Create the Systems table with schema corresponding to the options.
pub async fn create_table(pool: &SqlitePool /* TODO add options */) -> Result<(), Error> {
    // Default to playtest VBAM3 schema
    System::create_table(pool).await
}

#[cfg(test)]
mod tests {
    use crate::campaign::system::System;

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
        let mut rdr = csv::Reader::from_reader(SYSTEM_IMPORT);
        let mut count = 0;
        for result in rdr.records() {
            let record = result.unwrap();
            let val = System::new(
                record.get(0).unwrap(),
                record.get(1).unwrap(),
                record.get(2).unwrap().parse().unwrap(),
                record.get(3).unwrap().parse().unwrap(),
                record.get(4).unwrap().parse().unwrap(),
                record.get(5).unwrap().parse().unwrap(),
                record.get(6).unwrap().parse().unwrap());
            assert!(exp.contains(&val));
            count += 1;
        }
        assert_eq!(count, exp.len());
    }
}
