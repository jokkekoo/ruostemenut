use serde;
use serde::{Deserialize, Serialize};


pub struct Obj {
    pub kitchenName: String,
    #[serde(default)]
    pub kitchenId: u16,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub info: String,
}