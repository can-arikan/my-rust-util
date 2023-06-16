#![allow(non_snake_case)]
use std::{ env, time::Duration };

use duration_string::DurationString;

pub struct Util;

impl Util {
    pub fn loadOrPanic(path: &str, error: &str) -> String {
        return env::var(path).expect(error);
    }

    pub fn loadOrDefault(path: &str, default: &str) -> String {
        return match env::var(path) {
            Ok(env) => env,
            Err(_) => default.to_string(),
        };
    }

    pub fn loadOrNone(path: &str) -> Option<String> {
        return match env::var(path) {
            Ok(env) => Some(env),
            Err(_) => None,
        };
    }

    pub fn strToDuration(duration: String, err: &str) -> Duration {
        return DurationString::from_string(duration).expect(err).into();
    }
}
