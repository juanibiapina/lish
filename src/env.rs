use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use error::{Error, Result};
use types::LispValue;

#[derive(PartialEq, Debug)]
pub struct EnvType {
    data: HashMap<String, LispValue>,
    outer: Option<Env>,
}

pub type Env = Rc<RefCell<EnvType>>;

pub fn env_new(outer: Option<Env>) -> Env {
    Rc::new(RefCell::new(EnvType {
        data: HashMap::new(),
        outer: outer,
    }))
}

pub fn env_set(env: &Env, key: &str, value: LispValue) {
    env.borrow_mut().data.insert(key.to_owned(), value);
}

fn env_find(env: &Env, name: &str) -> Option<Env> {
    let envdata = env.borrow();
    if envdata.data.contains_key(name) {
        Some(env.clone())
    } else {
        match envdata.outer {
            Some(ref env) => env_find(env, name),
            None => None,
        }
    }
}

pub fn env_get(env: &Env, name: &str) -> Result<LispValue> {
    match env_find(env, name) {
        Some(env) => {
            match env.borrow().data.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(Error::UndefinedBinding(name.to_owned())),
            }
        },
        None => Err(Error::UndefinedBinding(name.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use types;
    use super::*;

    #[test]
    fn test_simple_env_lookup() {
        let env = env_new(None);

        env_set(&env, "b", types::integer(18));

        assert_eq!(env_get(&env, "b").unwrap(), types::integer(18));
    }

    #[test]
    fn test_lookup_on_outer_env() {
        let outer = env_new(None);
        env_set(&outer, "b", types::integer(18));

        let env = env_new(Some(outer));

        assert_eq!(env_get(&env, "b").unwrap(), types::integer(18));
    }

    #[test]
    fn test_set_in_env_doesnt_change_outer() {
        let outer = env_new(None);
        env_set(&outer, "b", types::integer(18));

        let env = env_new(Some(outer.clone()));
        env_set(&env, "b", types::integer(20));

        assert_eq!(env_get(&outer, "b").unwrap(), types::integer(18));
    }
}
