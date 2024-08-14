use crate::bindings::oracle::query::OracleQuery;
use cosmwasm_std::{CustomQuery, QueryRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectQuery {
    Oracle(OracleQuery),
}

impl CustomQuery for ConnectQuery {}

impl From<OracleQuery> for QueryRequest<ConnectQuery> {
    fn from(msg: OracleQuery) -> Self {
        QueryRequest::Custom(ConnectQuery::Oracle(msg))
    }
}
