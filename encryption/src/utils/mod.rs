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
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

pub struct CpAbeKeyWrapper<K>(pub K);

impl<K: Serialize + DeserializeOwned> CpAbeKeyWrapper<K> {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self.0).unwrap().into_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let json_str = String::from_utf8(bytes.to_vec()).unwrap();
        let cpabe_key = serde_json::from_str(&json_str).unwrap();
        CpAbeKeyWrapper(cpabe_key)
    }

    pub fn get_key(self) -> K {
        return self.0;
    }
}
