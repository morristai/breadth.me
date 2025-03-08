use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "factor_rs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub date: Date,
    pub quality: Decimal,
    pub value: Decimal,
    pub magnificent7: Decimal,
    pub growth: Decimal,
    pub momentum: Decimal,
    pub high_dividend: Decimal,
    pub market_risk: Decimal,
    pub real_estate: Decimal,
    pub small_cap: Decimal,
    pub min_vol: Decimal,
    pub high_yield_bonds: Decimal,
    pub interest_rate: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
