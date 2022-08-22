use crate::proxy::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {

}


#[derive(Serialize, Deserialize)]
pub struct Level {
    data: Data
}