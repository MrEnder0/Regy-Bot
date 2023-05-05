use std::fmt::Debug;

use crate::utils::logger::*;

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let data = LogData {
                    importance: "ERROR".to_string(),
                    message: format!("{}: {:?}", msg, err),
                };
                log_this(data);
                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T, ()> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                let data = LogData {
                    importance: "ERROR".to_string(),
                    message: msg.to_string(),
                };
                log_this(data);
                panic!("{}", msg);
            }
        }
    }
}
