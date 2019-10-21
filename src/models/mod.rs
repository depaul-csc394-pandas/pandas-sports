use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod sql;

#[derive(Deserialize, Serialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Sport {
    Baseball,
    Basketball,
    Football,
    Hockey,
    Soccer,
    Volleyball,
}
