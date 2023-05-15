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
use crate::cpabe::CpAbeExtensionScheme;
use crate::utils::CpAbeKeyWrapper;
use ink::prelude::vec::Vec;
use rabe::schemes::bsw::{setup, CpAbeMasterKey, CpAbePublicKey};

pub struct NotInitialized;

pub struct Initialized {
    public_key: CpAbePublicKey,
    master_key: CpAbeMasterKey,
}

pub struct CpAbeBuilder<State> {
    state: State,
}

impl Default for CpAbeBuilder<NotInitialized> {
    fn default() -> Self {
        Self {
            state: NotInitialized,
        }
    }
}

impl CpAbeBuilder<NotInitialized> {
    pub fn cpabe_setup() -> (Vec<u8>, Vec<u8>) {
        let (public_key, master_key) = setup();
        let public_key = CpAbeKeyWrapper(public_key);
        let master_key = CpAbeKeyWrapper(master_key);

        (public_key.to_bytes(), master_key.to_bytes())
    }

    pub fn keys(public_key: &[u8], master_key: &[u8]) -> CpAbeBuilder<Initialized> {
        let public_key = CpAbeKeyWrapper::<CpAbePublicKey>::from_bytes(public_key).get_key();
        let master_key = CpAbeKeyWrapper::<CpAbeMasterKey>::from_bytes(master_key).get_key();

        CpAbeBuilder {
            state: Initialized {
                public_key,
                master_key,
            },
        }
    }
}

impl CpAbeBuilder<Initialized> {
    pub fn build(self) -> CpAbeExtensionScheme {
        CpAbeExtensionScheme {
            public_key: self.state.public_key,
            master_key: self.state.master_key,
        }
    }
}
