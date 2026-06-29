#![no_std]

mod admin;
mod balance;
mod errors;
mod events;
pub mod nft;
mod storage;
mod vault;
pub mod interface;
pub mod example_consumer;

pub use nft::StakeReceiptNFT;
pub use vault::VaultContract;

use soroban_sdk::{contractimpl, Address, Env};

#[contractimpl]
impl interface::IStakingPool for VaultContract {
    fn staked_amount(env: Env, user: Address) -> i128 {
        let shares = balance::get_shares(&env, &user);
        if shares == 0 {
            return 0;
        }
        let total_shares = balance::get_total_shares(&env);
        let total_deposited = balance::get_total_deposited(&env);
        balance::shares_to_amount(total_shares, total_deposited, shares).unwrap_or(0)
    }

    fn pending_reward(env: Env, user: Address) -> i128 {
        VaultContract::calc_pending_reward(env, user).unwrap_or(0)
    }

    fn total_staked(env: Env) -> i128 {
        balance::get_total_deposited(&env)
    }

    fn is_paused(env: Env) -> bool {
        VaultContract::is_paused(env)
    }
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod test_integration;
