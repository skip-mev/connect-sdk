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

#[cfg(not(feature = "stargate"))]
impl From<OracleQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: OracleQuery) -> Self {
        QueryRequest::Custom(ConnectQuery::Oracle(msg))
    }
}

#[cfg(not(feature = "stargate"))]
impl From<MarketMapQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: MarketMapQuery) -> Self {
        QueryRequest::Custom(ConnectQuery::MarketMap(msg))
    }
}

#[cfg(feature = "stargate")]
impl From<MarketMapQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: MarketMapQuery) -> Self {
        use cosmwasm_std::to_json_binary;

        let data = to_json_binary(&msg).unwrap();
        let path = match msg {
            MarketMapQuery::Params {} => {
                "/slinky.marketmap.v1.Query/Params".to_string()
            }
            MarketMapQuery::LastUpdated {} => {
                "/slinky.marketmap.v1.Query/LastUpdated".to_string()
            }
            MarketMapQuery::MarketMap {} => {
                "/slinky.marketmap.v1.Query/MarketMap".to_string()
            }
            MarketMapQuery::Market { .. } => {
                "/slinky.marketmap.v1.Query/Market".to_string()
            }
        };

        QueryRequest::Stargate { path, data }
    }
}

#[cfg(feature = "stargate")]
impl From<OracleQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: OracleQuery) -> Self {
        use cosmwasm_std::to_json_binary;

        let data = to_json_binary(&msg).unwrap();
        let path = match msg {
            OracleQuery::GetAllCurrencyPairs {} => {
                "/slinky.oracle.v1.Query/GetAllCurrencyPairs".to_string()
            }
            OracleQuery::GetPrice { .. } => {
                "/slinky.oracle.v1.Query/GetPrice".to_string()
            }
            OracleQuery::GetPrices { .. } => {
                "/slinky.oracle.v1.Query/GetPrices".to_string()
            }
        };

        QueryRequest::Stargate { path, data }
    }
}
