use std::collections::HashMap;

use env::{Env, env_new, env_set};
use types::{LispValue, native_function};
use core::math;

fn ns() -> HashMap<&'static str, LispValue> {
    let mut ns = HashMap::new();;

    ns.insert("+", native_function(math::add));
    ns.insert("-", native_function(math::sub));

    ns
}

pub fn create() -> Env {
    let env = env_new();

    for (k, v) in ns().into_iter() {
        env_set(&env, k, v);
    }

    env
}
