use crate::{
    account::{
        AccountType,
        QuotaAccountType,
    },
    api::{
        Asset,
        API,
    },
    client::Client,
    errors::BybitError,
    model::{
        AllCoinsResponse,
        ConvertCommonResponse,
        QuoteApply,
        QuoteApplyResponse,
    },
    util::build_request,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct AssetManager {
    pub client: Client,
    pub recv_window: u16,
}

impl AssetManager {
    pub async fn query_all_coins<'b>(
        &self,
        account_type: AccountType,
        coins: Option<&[&str]>,
    ) -> Result<AllCoinsResponse, BybitError> {
        // Create a new BTreeMap to hold the request parameters.
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("accountType".into(), account_type.as_ref().into());

        // If a coin is specified, insert it into the parameters.
        if let Some(coins) = coins {
            let coins = coins.join(",");
            parameters.insert("coins".into(), coins);
        }

        // Build the request using the parameters.
        let req = build_request(&parameters);
        let response: ConvertCommonResponse<AllCoinsResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryAccountCoinBalance),
                self.recv_window.into(),
                Some(req),
            )
            .await?;

        Ok(response.result)
    }

    pub async fn apply_for_quota<'b>(
        &self,
        from_coin: &str,
        to_coin: &str,
        request_coin: &str,
        request_amount: String,
        account_type: QuotaAccountType,
    ) -> Result<QuoteApplyResponse, BybitError> {
        let req = QuoteApply::custom(
            from_coin,
            to_coin,
            request_coin,
            request_amount,
            account_type.as_ref(),
        );
        let request = serde_json::to_string(&req).unwrap();
        let response: ConvertCommonResponse<QuoteApplyResponse> = self
            .client
            .post_signed(
                API::Asset(Asset::RequestQuote),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        Ok(response.result)
    }
}
