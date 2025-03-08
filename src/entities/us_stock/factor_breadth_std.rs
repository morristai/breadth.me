use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "factor_breadth_std")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub date: Date,
    pub factor_std: Option<f64>,
    pub sector_std: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
