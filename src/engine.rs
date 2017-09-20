use std::fs::File;
use std::io::Read;
use std::env;

use shell::engine::Engine as ShellEngine;
use lisp::engine::Engine as LispEngine;
use error::Result;
use env::Env;
use core;
use types::LispValue;

pub struct Engine {
    lisp_engine: LispEngine,
    shell_engine: ShellEngine,
    env: Env,
}

impl Engine {
    pub fn new() -> Engine {
        let core_env = core::env::create();

        Engine {
            lisp_engine: LispEngine::new(),
            shell_engine: ShellEngine::new(),
            env: core_env,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<Option<LispValue>> {
        let first_char = input.chars().next();

        match first_char {
            None => Ok(None),
            Some(c) => {
                if c == '(' {
                    Ok(Some(self.lisp_engine.run(input, self.env.clone())?))
                } else {
                    self.shell_engine.run(input, self.env.clone())?;

                    Ok(None)
                }
            }
        }
    }

    pub fn load_standard_library(&mut self) -> Result<()> {
        let exe_path = env::current_exe()?;
        let mut dir = match exe_path.parent() {
            Some(path) => path.to_path_buf(),
            None => panic!("cant determine exe directory"),
        };

        dir.push("stdlib");
        dir.push("stdlib.lish");

        let stdlib_path = dir.to_str().unwrap();

        self.load_file(stdlib_path)
    }

    pub fn load_file(&mut self, file_name: &str) -> Result<()> {
        let mut buffer = String::new();

        File::open(file_name).and_then(|mut f| f.read_to_string(&mut buffer))?;

        self.run(&format!("(do {})", buffer))?;

        Ok(())
    }
}
