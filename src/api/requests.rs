use crate::api::response_object::TwoDayDiff;
use crate::config::AppState;
use crate::entities::us_stock::factor_breadth_std::Model as FactorBreadthStd;
use crate::entities::us_stock::factor_rs::Model as FactorRelativeStrength;
use crate::entities::us_stock::stock_info::Model as StockInfo;
use crate::entities::us_stock::stock_sector_ma20::Model as StockSectorMa20;
use crate::entities::us_stock::table_update_log::Model as TableUpdateLog;
use crate::entities::us_stock::*;
use crate::error::{Error, Result};
use axum::extract::{Query, State};
use axum::Json;
use cached::proc_macro::cached;
use crate::api::params::InfoParams;
use sea_orm::*;

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("stock_sector_ma20") }"#
)]
pub async fn two_day_diff(State(state): State<AppState>) -> Result<Json<TwoDayDiff>, Error> {
    let records = stock_sector_ma20::Entity::find()
        .order_by_desc(stock_sector_ma20::Column::Date)
        .limit(2)
        .all(&state.db)
        .await?;

    let t1 = &records[0];
    let t2 = &records[1];

    let diff = TwoDayDiff {
        date: t1.date,
        mat: (t1.mat as i8) - (t2.mat as i8),
        com: (t1.com as i8) - (t2.com as i8),
        cns: (t1.cns as i8) - (t2.cns as i8),
        cnd: (t1.cnd as i8) - (t2.cnd as i8),
        ene: (t1.ene as i8) - (t2.ene as i8),
        fin: (t1.fin as i8) - (t2.fin as i8),
        hlt: (t1.hlt as i8) - (t2.hlt as i8),
        ind: (t1.ind as i8) - (t2.ind as i8),
        rei: (t1.rei as i8) - (t2.rei as i8),
        tec: (t1.tec as i8) - (t2.tec as i8),
        utl: (t1.utl as i8) - (t2.utl as i8),
        total: (t1.total as i8) - (t2.total as i8),
    };

    Ok(Json(diff))
}

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("factor_sector_sd") }"#
)]
pub async fn factor_sector_std(State(state): State<AppState>) -> Result<Json<Vec<FactorBreadthStd>>, Error> {
    let result = factor_breadth_std::Entity::find()
        .order_by_desc(factor_breadth_std::Column::Date)
        .all(&state.db)
        .await?;

    Ok(Json(result))
}

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("sector_breadth") }"#
)]
pub async fn stock_sector_breadth(State(state): State<AppState>) -> Result<Json<StockSectorMa20>, Error> {
    let result = stock_sector_ma20::Entity::find()
        .order_by_desc(stock_sector_ma20::Column::Date)
        .limit(1)
        .one(&state.db)
        .await?;

    Ok(Json(result.unwrap()))
}

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("breadth_trend") }"#
)]
pub async fn stock_sector_breadth_trend(State(state): State<AppState>) -> Result<Json<Vec<StockSectorMa20>>, Error> {
    let result = stock_sector_ma20::Entity::find()
        .order_by_desc(stock_sector_ma20::Column::Date)
        .all(&state.db)
        .await?;

    Ok(Json(result))
}

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("factor_rs") }"#
)]
pub async fn factor_relative_strength(
    State(state): State<AppState>,
) -> Result<Json<Vec<FactorRelativeStrength>>, Error> {
    let rows = factor_rs::Entity::find().all(&state.db).await?;

    Ok(Json(rows))
}

#[cached(
    time = 2700,
    result = true,
    key = "String",
    convert = r#"{ String::from("stock_daily") }"#
)]
pub async fn last_update_time(State(state): State<AppState>) -> Result<Json<TableUpdateLog>, Error> {
    let result = table_update_log::Entity::find()
        .filter(table_update_log::Column::TableName.eq("stock_daily"))
        .one(&state.db)
        .await?;

    Ok(Json(result.unwrap()))
}

#[cached(
    time = 43200,
    result = true,
    key = "String",
    convert = r#"{ format!("stock_info_{:?}", params) }"#
)]
pub async fn company_list(
    Query(params): Query<InfoParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<StockInfo>>, Error> {
    let sector = params.sector.unwrap_or(vec!["all".to_string()]);
    let industry = params.industry.unwrap_or(vec!["all".to_string()]);

    let mut query = stock_info::Entity::find();

    if sector != vec!["all"] {
        query = query.filter(stock_info::Column::SpSector.is_in(sector));
    }

    if industry != vec!["all"] {
        query = query.filter(stock_info::Column::Industry.is_in(industry));
    }

    let result = query.all(&state.db).await?;

    Ok(Json(result))
}

pub async fn health_check(State(state): State<AppState>) -> Result<(), Error> {
    state.db.ping().await?;
    Ok(())
}
