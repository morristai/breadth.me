use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "table_update_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub table_name: String,
    pub last_update_time: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
