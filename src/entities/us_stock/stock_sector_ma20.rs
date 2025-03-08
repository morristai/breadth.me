use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "stock_sector_ma20")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub date: Date,
    pub mat: u8,
    pub com: u8,
    pub cns: u8,
    pub cnd: u8,
    pub ene: u8,
    pub fin: u8,
    pub hlt: u8,
    pub ind: u8,
    pub rei: u8,
    pub tec: u8,
    pub utl: u8,
    pub total: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
