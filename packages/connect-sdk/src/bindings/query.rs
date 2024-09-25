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
                "/connect.marketmap.v2.Query/Params".to_string()
            }
            MarketMapQuery::LastUpdated {} => {
                "/connect.marketmap.v2.Query/LastUpdated".to_string()
            }
            MarketMapQuery::MarketMap {} => {
                "/connect.marketmap.v2.Query/MarketMap".to_string()
            }
            MarketMapQuery::Market { .. } => {
                "/connect.marketmap.v2.Query/Market".to_string()
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
                "/connect.oracle.v2.Query/GetAllCurrencyPairs".to_string()
            }
            OracleQuery::GetPrice { .. } => {
                "/connect.oracle.v2.Query/GetPrice".to_string()
            }
            OracleQuery::GetPrices { .. } => {
                "/connect.oracle.v2.Query/GetPrices".to_string()
            }
        };

        QueryRequest::Stargate { path, data }
    }
}
