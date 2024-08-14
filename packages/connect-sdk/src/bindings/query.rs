use crate::bindings::{
    marketmap::query::MarketMapQuery, oracle::query::OracleQuery,
};
use cosmwasm_std::{CustomQuery, QueryRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectQuery {
    Oracle(OracleQuery),
    MarketMap(MarketMapQuery),
}

impl CustomQuery for ConnectQuery {}

impl From<OracleQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: OracleQuery) -> Self {
        QueryRequest::Custom(ConnectQuery::Oracle(msg))
    }
}

impl From<MarketMapQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: MarketMapQuery) -> Self {
        QueryRequest::Custom(ConnectQuery::MarketMap(msg))
    }
}
