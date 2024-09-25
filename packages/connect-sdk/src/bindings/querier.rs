use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use super::{
    marketmap::query::{
        LastUpdatedResponse, MarketMapQuery, MarketMapResponse, MarketResponse,
        ParamsResponse,
    },
    oracle::query::{
        GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse,
        OracleQuery,
    },
    query::ConnectQuery,
};

pub struct ConnectQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ConnectQuery>,
}

impl<'a> ConnectQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<ConnectQuery>) -> Self {
        ConnectQuerier { querier }
    }

    pub fn get_oracle_price(
        &self,
        currency_pair: String
    ) -> StdResult<GetPriceResponse> {
        let request: QueryRequest<ConnectQuery> = OracleQuery::GetPrice {
            currency_pair,
        }.into();
        self.querier.query::<GetPriceResponse>(&request.into())
    }

    pub fn get_oracle_prices(
        &self,
        currency_pair_ids: Vec<String>,
    ) -> StdResult<GetPricesResponse> {
        let request: QueryRequest<ConnectQuery> =
            OracleQuery::GetPrices { currency_pair_ids }.into();
        self.querier.query::<GetPricesResponse>(&request.into())
    }

    pub fn get_oracle_all_currency_pairs(
        &self,
    ) -> StdResult<GetAllCurrencyPairsResponse> {
        let request: QueryRequest<ConnectQuery> = OracleQuery::GetAllCurrencyPairs {}.into();
        self.querier
            .query::<GetAllCurrencyPairsResponse>(&request.into())
    }

    pub fn get_marketmap_params(&self) -> StdResult<ParamsResponse> {
        let request: QueryRequest<ConnectQuery> = MarketMapQuery::Params {}.into();
        self.querier.query::<ParamsResponse>(&request.into())
    }

    pub fn get_marketmap_last_updated(&self) -> StdResult<LastUpdatedResponse> {
        let request:QueryRequest<ConnectQuery>  = MarketMapQuery::LastUpdated {}.into();
        self.querier.query::<LastUpdatedResponse>(&request.into())
    }

    pub fn get_marketmap_market_map(&self) -> StdResult<MarketMapResponse> {
        let request: QueryRequest<ConnectQuery> = MarketMapQuery::MarketMap {}.into();
        self.querier.query::<MarketMapResponse>(&request.into())
    }

    pub fn get_marketmap_market(
        &self,
        base: String,
        quote: String,
    ) -> StdResult<MarketResponse> {
        let request: QueryRequest<ConnectQuery> = MarketMapQuery::Market {
            currency_pair: super::marketmap::types::CurrencyPair {
                base,
                quote,
            },
        }.into();
        self.querier.query::<MarketResponse>(&request.into())
    }
}
