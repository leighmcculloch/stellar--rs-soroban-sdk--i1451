#![no_std]
use soroban_sdk::{contracttrait, symbol_short, Env};

#[contracttrait]
pub trait Pause : PauseHooks {
    fn pause(env: &Env, paused: bool) {
        Self::pause_before(env);
        env.storage().persistent().set(&symbol_short!("paused"), &paused)
    }

    fn paused(env: &Env) -> bool {
        env.storage().persistent().get(&symbol_short!("paused")).unwrap_or(false)
    }
}

pub trait PauseHooks {
    fn pause_before(env: &Env);
}
