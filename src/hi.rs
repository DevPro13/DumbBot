use serde::{Serialize};
// /hi responce
#[derive(Serialize,Debug,Default)]
pub struct Hello{
    value:String,
}
impl Hello{
    pub fn responce_hi(&mut self)->Hello{
        Hello{value:"hello".to_string(),}
    }
}