use crate::msg::QueryMsg;
use connect_sdk::bindings::oracle::query::{
    GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse,
    OracleQuery,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::to_json_binary;
use cosmwasm_std::{Binary, Deps, Env, QueryRequest, StdResult};

use connect_sdk::bindings::query::ConnectQuery;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { base, quote } => {
            to_json_binary(&query_price(deps, base, quote)?)
        }
        QueryMsg::Prices { currency_pair_ids } => {
            to_json_binary(&query_prices(deps, currency_pair_ids)?)
        }
        QueryMsg::CurrencyPairs {} => {
            to_json_binary(&query_currency_pairs(deps)?)
        }
    }
}

fn query_currency_pairs(deps: Deps) -> StdResult<GetAllCurrencyPairsResponse> {
    todo!()
}

fn query_prices(
    deps: Deps,
    currency_pair_ids: Vec<String>,
) -> StdResult<GetPricesResponse> {
    todo!()
}

fn query_price(
    deps: Deps,
    base: String,
    quote: String,
) -> StdResult<GetPriceResponse> {
    todo!()
}
