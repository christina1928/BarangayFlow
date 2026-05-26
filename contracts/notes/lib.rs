// lib.rs
#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Address,
    Env,
};

#[derive(Clone)]
#[contracttype]
pub struct Beneficiary {
    pub amount: i128,
    pub claimed: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Beneficiary(Address),
}

#[contract]
pub struct BarangayFlowContract;

#[contractimpl]
impl BarangayFlowContract {

    pub fn init(env: Env, admin: Address) {
        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
    }

    pub fn add_beneficiary(
        env: Env,
        admin: Address,
        user: Address,
        amount: i128,
    ) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("unauthorized");
        }

        let beneficiary = Beneficiary {
            amount,
            claimed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Beneficiary(user), &beneficiary);
    }

    pub fn claim(env: Env, user: Address) {
        user.require_auth();

        let key = DataKey::Beneficiary(user.clone());

        let mut beneficiary: Beneficiary = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if beneficiary.claimed {
            panic!("already claimed");
        }

        beneficiary.claimed = true;

        env.storage()
            .persistent()
            .set(&key, &beneficiary);
    }

    pub fn get_status(
        env: Env,
        user: Address,
    ) -> Beneficiary {
        env.storage()
            .persistent()
            .get(&DataKey::Beneficiary(user))
            .unwrap()
    }
}