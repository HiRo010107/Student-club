#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol,
};

#[contract]
pub struct StudentClub;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Member(Address),
    Count,
}

#[contractimpl]
impl StudentClub {
    // Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Count, &0u32);
    }

    // Add a member
    pub fn add_member(env: Env, caller: Address, member: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        caller.require_auth();

        if caller != admin {
            panic!("Only admin");
        }

        let key = DataKey::Member(member.clone());

        if !env.storage().persistent().has(&key) {
            env.storage().persistent().set(&key, &true);

            let count: u32 = env
                .storage()
                .instance()
                .get(&DataKey::Count)
                .unwrap_or(0);

            env.storage().instance().set(&DataKey::Count, &(count + 1));
        }
    }

    // Remove a member
    pub fn remove_member(env: Env, caller: Address, member: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        caller.require_auth();

        if caller != admin {
            panic!("Only admin");
        }

        let key = DataKey::Member(member.clone());

        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);

            let count: u32 = env
                .storage()
                .instance()
                .get(&DataKey::Count)
                .unwrap_or(0);

            if count > 0 {
                env.storage().instance().set(&DataKey::Count, &(count - 1));
            }
        }
    }

    // Check membership
    pub fn is_member(env: Env, member: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Member(member))
    }

    // Get admin
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap()
    }

    // Total members
    pub fn member_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Count)
            .unwrap_or(0)
    }
}