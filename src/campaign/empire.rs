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

#[allow(unused)]
#[derive(sqlx::FromRow)]
pub struct Empire {
    pub id: i64,
    pub name: String,
    pub treasury: i32,
    pub tech: i32,
}

impl Empire {
    /// Create a new empire.
    #[allow(unused)]
    pub fn new(name: &str) -> Empire {
        Self {
            id: 0,
            name: name.to_string(),
            treasury: 0,
            tech: 0,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::campaign::empire::Empire;

    pub fn empires() -> Vec<Empire> {
        let mut emp = Vec::new();
        emp.push(Empire::new("Senorian"));
        emp.push(Empire::new("Human"));
        emp.push(Empire::new("Kili"));
        emp.push(Empire::new("Loran"));
        emp.push(Empire::new("Jain"));
        emp.push(Empire::new("Brindaki"));
        emp.push(Empire::new("Graal"));
        emp.push(Empire::new("Tirelon"));
        emp
    }
    // TODO Add tests
}
