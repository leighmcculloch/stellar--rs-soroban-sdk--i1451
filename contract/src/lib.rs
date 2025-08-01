#![no_std]
use library::{Pause, PauseHooks};
use soroban_sdk::{contract, contracttrait, Env};

#[contract]
pub struct Contract;

#[contracttrait]
impl Pause for Contract {}

impl PauseHooks for Contract {
    fn pause_before(_: &Env) {
        // TODO: Auth, etc.
    }
}
