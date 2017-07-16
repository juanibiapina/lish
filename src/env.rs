use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use error::{Error, Result};
use types::LispValue;

#[derive(PartialEq, Debug)]
pub struct EnvType {
    data: HashMap<String, LispValue>,
}

pub type Env = Rc<RefCell<EnvType>>;

pub fn env_new() -> Env {
    Rc::new(RefCell::new(EnvType {
        data: HashMap::new(),
    }))
}

pub fn env_set(env: &Env, key: &str, value: LispValue) {
    env.borrow_mut().data.insert(key.to_owned(), value);
}

pub fn env_get(env: &Env, name: &str) -> Result<LispValue> {
    match env.borrow().data.get(name) {
        Some(value) => Ok(value.clone()),
        None => Err(Error::UndefinedBinding(name.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use types;
    use super::*;

    #[test]
    fn test_env() {
        let env = env_new();

        env_set(&env, "b", types::integer(18));

        assert_eq!(env_get(&env, "b").unwrap(), types::integer(18));
    }
}
