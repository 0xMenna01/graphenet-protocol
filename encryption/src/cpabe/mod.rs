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

mod attributes;

use attributes::AttributeSet;
use ink::primitives::AccountId;
use rabe::schemes::bsw::{CpAbeMasterKey, CpAbePublicKey, CpAbeSecretKey};

pub enum Error {
    AttributeCreationFailed,
    NotAllowed,
}

pub trait CpAbeScheme {
    fn setup() -> (CpAbePublicKey, CpAbeMasterKey);
    fn create_id_for_attributes(account: AccountId, attributes: AttributeSet) -> Result<(), Error>;
    fn get_key() -> Result<CpAbeSecretKey, Error>;
}
