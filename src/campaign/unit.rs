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

impl GroundType {}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct GroundUnit {
    id: i64,
    gtype: i64,
    loc: i64,
}

impl GroundUnit {}

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

impl ShipType {}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct Ship {
    id: i64,
    stype: i64,
    fleet: i64,
    crip: bool,
    moth: bool,
}

impl Ship {}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct Fleet {
    id: i64,
    name: String,
    owner: i64,
    location: i64,
}

impl Fleet {}

#[cfg(test)]
mod tests {
    // TODO Add tests
}
