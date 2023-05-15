//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::{string::String, vec::Vec};

type Attribute = String;

#[ink::storage_item]
pub struct AttributeSet {
    attributes: Vec<Attribute>,
}

impl AttributeSet {
    pub fn new() -> Self {
        let attributes = Vec::new();
        Self { attributes }
    }

    pub fn add(&mut self, attribute: Attribute) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute)
        }
    }

    pub fn add_all(&mut self, attributes: &mut Vec<Attribute>) {
        self.attributes.append(attributes);
    }

    pub fn get(&self, i: usize) -> Option<&Attribute> {
        self.attributes.get(i)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }

    pub fn len(&self) -> usize {
        self.attributes.len()
    }
}
