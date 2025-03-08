use crate::entities::us_stock::table_update_log::Model as TableUpdateLog;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LastUpdateTime {
    pub last_update_time: Option<DateTime<Utc>>,
}

impl From<TableUpdateLog> for LastUpdateTime {
    fn from(model: TableUpdateLog) -> Self {
        Self {
            last_update_time: model.last_update_time,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TwoDayDiff {
    pub date: NaiveDate,
    pub mat: i8,
    pub com: i8,
    pub cns: i8,
    pub cnd: i8,
    pub ene: i8,
    pub fin: i8,
    pub hlt: i8,
    pub ind: i8,
    pub rei: i8,
    pub tec: i8,
    pub utl: i8,
    pub total: i8,
}
