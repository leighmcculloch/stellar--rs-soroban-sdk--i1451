#![no_std]
use soroban_sdk::{contract, contractimpl, Env};
use library::{Pause, PauseHooks, contractimpl_for_pause};

#[contract]
pub struct Contract;

#[contractimpl]
impl Pause for Contract {}

impl PauseHooks for Contract {
    fn pause_before(_: &Env) {
        // TODO: Auth, etc.
    }
}
