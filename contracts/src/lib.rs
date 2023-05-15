// Copyright (C) 2022-2023 Graphenet
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

#[openbrush::contract]
pub mod cpabe_contract {
    use encryption::cpabe::builder::CpAbeBuilder;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct CpAbeExt {
        public_key: Vec<u8>,
        master_key: Vec<u8>,
    }

    impl CpAbeExt {
        #[ink(constructor)]
        pub fn default() -> Self {
            let (public_key, master_key) = CpAbeBuilder::cpabe_setup();

            Self {
                public_key,
                master_key,
            }
        }

        #[ink(message)]
        pub fn todo(&self) {}
    }
}
