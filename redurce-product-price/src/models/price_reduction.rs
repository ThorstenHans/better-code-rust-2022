use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReductionModel {
    pub id: i32,
    pub reduce_by: f32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedReductionModel {
    pub id: i32,
    pub reduce_by: f32,
    pub verified: bool,
}

impl VerifiedReductionModel {
    pub fn new(id: i32, reduce_by: f32, verified: bool) -> VerifiedReductionModel {
        VerifiedReductionModel {
            id,
            reduce_by,
            verified,
        }
    }
}
