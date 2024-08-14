use cosmwasm_std::{QuerierWrapper, StdResult};

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
        base: String,
        quote: String,
    ) -> StdResult<GetPriceResponse> {
        let request = ConnectQuery::Oracle(OracleQuery::GetPrice {
            currency_pair: super::oracle::types::CurrencyPair { base, quote },
        });
        self.querier.query::<GetPriceResponse>(&request.into())
    }

    pub fn get_oracle_prices(
        &self,
        currency_pair_ids: Vec<String>,
    ) -> StdResult<GetPricesResponse> {
        let request =
            ConnectQuery::Oracle(OracleQuery::GetPrices { currency_pair_ids });
        self.querier.query::<GetPricesResponse>(&request.into())
    }

    pub fn get_oracle_all_currency_pairs(
        &self,
    ) -> StdResult<GetAllCurrencyPairsResponse> {
        let request = ConnectQuery::Oracle(OracleQuery::GetAllCurrencyPairs {});
        self.querier
            .query::<GetAllCurrencyPairsResponse>(&request.into())
    }

    pub fn get_marketmap_params(&self) -> StdResult<ParamsResponse> {
        let request = ConnectQuery::MarketMap(MarketMapQuery::Params {});
        self.querier.query::<ParamsResponse>(&request.into())
    }

    pub fn get_marketmap_last_updated(&self) -> StdResult<LastUpdatedResponse> {
        let request = ConnectQuery::MarketMap(MarketMapQuery::LastUpdated {});
        self.querier.query::<LastUpdatedResponse>(&request.into())
    }

    pub fn get_marketmap_market_map(&self) -> StdResult<MarketMapResponse> {
        let request = ConnectQuery::MarketMap(MarketMapQuery::MarketMap {});
        self.querier.query::<MarketMapResponse>(&request.into())
    }

    pub fn get_marketmap_market(
        &self,
        base: String,
        quote: String,
    ) -> StdResult<MarketResponse> {
        let request = ConnectQuery::MarketMap(MarketMapQuery::Market {
            currency_pair: super::marketmap::types::CurrencyPair {
                base,
                quote,
            },
        });
        self.querier.query::<MarketResponse>(&request.into())
    }
}
