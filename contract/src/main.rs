#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use muta_std::{debug, default_alloc, entry, error::SysError, high_level};

#[derive(Debug, Clone)]
pub enum Error {
    SysError(SysError),
    // user defined errors here
    ArgsErr,
}

impl Error {
    fn to_error(&self) -> (u64, String) {
        match self {
            Self::SysError(_e) => (1, alloc::format!("{:?}", self)),
            Self::ArgsErr => (
                2,
                "wrong args, should be like 'set [key] [value]' or 'get [key]'".to_string(),
            ),
        }
    }
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        Self::SysError(err)
    }
}

/// A simple storage implementation
pub fn main() -> Result<String, Error> {
    let args = high_level::load_args_string()?;
    debug!("args: {:?}", args);

    let cmd: Vec<&str> = args.split(' ').collect();
    debug!("cmd: {:?}", cmd);

    if cmd.len() < 1 {
        return Err(Error::ArgsErr);
    }
    match cmd[0] {
        "set" => {
            if cmd.len() != 3 {
                return Err(Error::ArgsErr);
            }
            high_level::set_storage_string(cmd[1], cmd[2]);
            Ok("".to_string())
        }
        "get" => {
            if cmd.len() != 2 {
                return Err(Error::ArgsErr);
            }
            let val = high_level::get_storage_string(cmd[1])?;
            Ok(val)
        }
        _ => Err(Error::ArgsErr),
    }
}

pub fn _main() -> Result<String, (u64, String)> {
    main().map_err(|e| e.to_error())
}

entry!(_main);
default_alloc!();
