use std::collections::HashMap;

use env::{Env, env_new, env_set};
use types::{LispValue, native_function};
use core::math;
use core::file;

fn ns() -> HashMap<&'static str, LispValue> {
    let mut ns = HashMap::new();;

    ns.insert("+", native_function(math::add));
    ns.insert("-", native_function(math::sub));
    ns.insert("slurp", native_function(file::slurp));

    ns
}

pub fn create() -> Env {
    let env = env_new(None);

    for (k, v) in ns().into_iter() {
        env_set(&env, k, v);
    }

    env
}
