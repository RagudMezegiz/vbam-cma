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

#[allow(unused)]
#[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq)]
pub struct System {
    pub id: i64,
    pub name: String,
    pub ptype: String,
    pub raw: i32,
    pub cap: i32,
    pub pop: i32,
    pub mor: i32,
    pub ind: i32,
    pub dev: i32,
    pub fails: i32,
    pub owner: i64,
    #[sqlx(default)]
    pub owner_name: String,
}

impl System {
    /// Convert to string as a row of tab-separated fields.
    pub fn as_row(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.name,
            self.ptype,
            self.raw,
            self.cap,
            self.pop,
            self.mor,
            self.ind,
            self.dev,
            self.fails,
            self.owner_name
        )
    }

    /// Read systems from a CSV reader.
    pub fn read_csv<R>(mut rdr: csv::Reader<R>) -> Result<Vec<System>, String>
    where
        R: io::Read,
    {
        let mut v = Vec::new();
        for result in rdr.records() {
            match result {
                Ok(rcd) => {
                    if let Ok(sys) = Self::from_csv(rcd) {
                        v.push(sys)
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(v)
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

    // Create a new system.
    fn new(name: &str, ptype: &str, raw: i32, cap: i32, pop: i32, mor: i32, ind: i32) -> System {
        Self {
            id: 0,
            name: name.to_string(),
            ptype: ptype.to_string(),
            raw,
            cap,
            pop,
            mor,
            ind,
            dev: 0,
            fails: 0,
            owner: 0,
            owner_name: "None".to_string(),
        }
    }
}

/// Load a set of systems from a CSV file. Columns should be in order:
/// NAME,TYPE,RAW,CAP,POP,MOR,IND
pub fn read_from_csv(file: &str) -> Result<Vec<System>, String> {
    let r = match csv::Reader::from_path(file) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    System::read_csv(r)
}

#[cfg(test)]
pub mod tests {
    use crate::campaign::system::System;
    use csv::Reader;

    const SYSTEM_IMPORT: &[u8] = "NAME,TYPE,RAW,CAP,POP,MOR,IND\n\
        Senor Prime,HW,5,12,10,8,10\n\
        Vadurrinia,Adaptable,3,8,4,3,3\n\
        Zev'rch,Barren,2,6,3,2,2\n\
        Tibron,Barren,4,6,3,2,3\n"
        .as_bytes();

    pub fn systems() -> Vec<System> {
        let mut sys = Vec::new();
        sys.push(System::new("Senor Prime", "HW", 5, 12, 10, 8, 10));
        sys.push(System::new("Vadurrinia", "Adaptable", 3, 8, 4, 3, 3));
        sys.push(System::new("Zev'rch", "Barren", 2, 6, 3, 2, 2));
        sys.push(System::new("Tibron", "Barren", 4, 6, 3, 2, 3));
        sys
    }

    #[test]
    fn deserialize() {
        let exp = systems();
        let rdr = Reader::from_reader(SYSTEM_IMPORT);
        let act = System::read_csv(rdr).unwrap();
        assert_eq!(exp.len(), act.len());
        for sys in act {
            assert!(exp.contains(&sys));
        }
    }
}
