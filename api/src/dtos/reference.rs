use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct LinkType {
    pub id: Uuid,
    pub name: String,
}
