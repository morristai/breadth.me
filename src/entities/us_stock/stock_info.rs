use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "stock_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub code: String,
    pub name: String,
    pub sector: Option<String>,
    pub sp_sector: Option<String>,
    pub industry: Option<String>,
    pub country: Option<String>,
    pub total_cap: Option<Decimal>,
    pub is_spx: Option<i8>,
    pub spx_weight: Option<Decimal>,
    pub is_ndx: Option<i8>,
    pub ndx_weight: Option<Decimal>,
    pub is_dji: Option<i8>,
    pub dji_weight: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
