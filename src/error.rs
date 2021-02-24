//! This module contains enum Error.
//! Error type represents all possible errors that can occur when dealing
//! with the generic or any dedicated-exchange API

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ServiceUnavailable,
    BadParse,
    InvalidLogin,
    InvalidArguments,
    RateLimitExceeded,
    PairUnsupported,
    ExchangeSpecificError(String),
    UndefinedError,
