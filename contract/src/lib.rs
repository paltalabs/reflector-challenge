#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

mod storage;

use storage::{
    extend_instance_ttl, 
    set_initialized
};

#[contract]
struct ReflectoyChallenge;

#[contractimpl]
impl ReflectoyChallenge {
    fn __constructor(
        e: Env,
    ) {
        set_initialized(&e);
    }

    fn hello_world(e: Env) -> String {
        extend_instance_ttl(&e);

        String::from_str(&e, "Hello world")
    }
}