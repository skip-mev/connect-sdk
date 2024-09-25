use crate::msg::QueryMsg;
use connect_sdk::bindings::oracle::query::{
    GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse,
};
use connect_sdk::bindings::querier::ConnectQuerier;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, DepsMut, Empty, MessageInfo, Response};
use cosmwasm_std::{Binary, Deps, Env, StdResult};
use connect_sdk::bindings::oracle::types::CurrencyPair;
use connect_sdk::bindings::query::ConnectQuery;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut<ConnectQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<ConnectQuery>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { currency_pair } => {
            to_json_binary(&query_price(deps, currency_pair)?)
        }
        QueryMsg::Prices { currency_pair_ids } => {
            to_json_binary(&query_prices(deps, currency_pair_ids)?)
        }
        QueryMsg::CurrencyPairs {} => {
            to_json_binary(&query_currency_pairs(deps)?)
        }
    }
}

fn query_currency_pairs(
    deps: Deps<ConnectQuery>,
) -> StdResult<GetAllCurrencyPairsResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_all_currency_pairs()
}

fn query_prices(
    deps: Deps<ConnectQuery>,
    currency_pair_ids: Vec<String>,
) -> StdResult<GetPricesResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_prices(currency_pair_ids)
}

fn query_price(
    deps: Deps<ConnectQuery>,
    currency_pair: String
) -> StdResult<GetPriceResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_price(currency_pair)
}
