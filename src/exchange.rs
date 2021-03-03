//! This module contains Exchange enum.

use serde_json::value::Map;
use serde_json::value::Value;

use std::fmt::Debug;

use error::Error;
use pair::Pair;
use types::Ticker;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Excha