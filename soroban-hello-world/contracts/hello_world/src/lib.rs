#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};


pub struct LinkTreeContract;

#[derive(Default)]
pub struct LinkTreeData {
    insta_id: Symbol,
    discord_id: Symbol,
    linkedin_id: Symbol,
    twitter_id: Symbol,
    stellar_id: Symbol,
}

#[contractimpl]
impl LinkTreeContract {
    pub fn set_links(env: Env, account_id: Symbol, insta_id: Symbol, discord_id: Symbol, linkedin_id: Symbol, twitter_id: Symbol, stellar_id: Symbol) {
        let mut data: Map<Symbol, LinkTreeData> = env.storage().persistent().get(&Symbol::new("links")).unwrap_or_default();
        data.insert(account_id, LinkTreeData {
            insta_id,
            discord_id,
            linkedin_id,
            twitter_id,
            stellar_id,
        });
        env.storage().persistent().set(&Symbol::new("links"), data);
    }

    pub fn get_links(env: Env, account_id: Symbol) -> LinkTreeData {
        let data: Map<Symbol, LinkTreeData> = env.storage().persistent().get(&Symbol::new("links")).unwrap();
        data.get(&account_id).unwrap()
    }
}

#[cfg(test)]
mod test;
