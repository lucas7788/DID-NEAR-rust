use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};

mod basic;
use basic::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct DID {
    status: UnorderedMap<String, Status>,
    contexts: UnorderedMap<String, Vec<String>>,
    public_key: UnorderedMap<String, Vec<PublicKey>>,
    authentication: UnorderedMap<String, Vec<u32>>,
    controller: UnorderedMap<String, Vec<String>>,
    service: UnorderedMap<String, Service>,
    created: UnorderedMap<String, u64>,
    updated: UnorderedMap<String, u64>,
}

#[near_bindgen]
impl DID {
    pub fn reg_did_using_account(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        let status = self.status.get(&account_id);
        assert!(status.is_none());

        self.status.insert(&account_id, &Status::VALID);
        self.public_key.insert(
            &account_id,
            &vec![PublicKey::new_pk_and_auth(&account_id, account_pk)],
        );
        let index: u32 = 0;
        self.authentication.insert(&account_id, &vec![index]);
        self.created.insert(&account_id, &env::block_timestamp());

        let log_message = format!("reg_did_using_account: {}", &account_id);
        env::log(log_message.as_bytes());
    }

    pub fn deactive_did(&mut self) {
        let account_id = env::signer_account_id();

        let status = self.status.get(&account_id);
        assert!(status.is_some());

        self.status.insert(&account_id, &Status::DeActive);
        self.contexts.remove(&account_id);
        self.public_key.remove(&account_id);
        self.authentication.remove(&account_id);
        self.controller.remove(&account_id);
        self.service.remove(&account_id);
        self.created.remove(&account_id);
        self.updated.remove(&account_id);

        let log_message = format!("deactive_did: {}", &account_id);
        env::log(log_message.as_bytes());
    }

    pub fn add_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        let status = self.status.get(&account_id).unwrap();
        match status {
            Status::VALID => (),
            _ => env::panic(b"did status is not valid"),
        };

        let mut public_key_list = self.public_key.get(&account_id).unwrap();
        public_key_list.push(PublicKey::new_pk(&account_id, pk.clone()));
        self.created.insert(&account_id, &env::block_timestamp());

        let log_message = format!("add_key, id:{}, public key: {:?}, controller: {}", &account_id, pk, controller);
        env::log(log_message.as_bytes());
    }

    pub fn remove_key(&mut self, pk: Vec<u8>) {}

    pub fn add_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let did = gen_did(&account_id);
        self.service.insert(&did, &ser);
    }
}

fn gen_did(account_id: &str) -> String {
    String::from("did:near:") + account_id
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = DID::default();
    }
}
