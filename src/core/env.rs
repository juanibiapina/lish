use std::collections::HashMap;

use env::{Env, env_new, env_set};
use types::{LispValue, native_function};
use core::math;
use core::file;
use core::reader;
use core::list;
use core::string;
use core::hash;
use core::printer;
use core::environment;

fn ns() -> HashMap<&'static str, LispValue> {
    let mut ns = HashMap::new();

    ns.insert("+", native_function(math::add));
    ns.insert("-", native_function(math::sub));

    ns.insert("slurp", native_function(file::slurp));
    ns.insert("read", native_function(reader::read));

    ns.insert("list", native_function(list::list_c));

    ns.insert("string-append", native_function(string::string_append));

    ns.insert("hash", native_function(hash::hash));
    ns.insert("hash-set", native_function(hash::hash_set));

    ns.insert("display", native_function(printer::display));
    ns.insert("write", native_function(printer::write));

    ns.insert("env-get", native_function(environment::env_get));
    ns.insert("env-set", native_function(environment::env_set));

    ns
}

pub fn create() -> Env {
    let env = env_new(None);

    for (k, v) in ns().into_iter() {
        env_set(&env, k, v);
    }

    env
}
