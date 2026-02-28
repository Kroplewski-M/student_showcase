use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct LinkTypes {
    pub id: Uuid,
    pub name: String,
}
