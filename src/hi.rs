mod api_rust_data;
use super::api_rust_data::{Hello,};
impl Hello{
    fn responce_hi(&mut self)->Hello{
        Hello{value:"hello".to_string(),}
    }
}