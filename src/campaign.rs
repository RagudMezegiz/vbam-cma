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

//! The program interface to the back-end data layer.

/// A Campaign, in addition to having the same meaning as in the VBAM rules,
/// is the control layer managing the conduct of the game itself. Every
/// campaign has a name which is used as the name of the backend database.
pub struct Campaign {
    name: String,
}

impl Campaign {
    /// Create a new campaign.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
