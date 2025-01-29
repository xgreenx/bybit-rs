use bybit::{
    account::{
        AccountType,
        QuotaAccountType,
    },
    api::*,
    asset::AssetManager,
    enable_tracing,
    test_utils::{
        api_key,
        secret,
    },
};
use tokio;

enable_tracing!();

#[tokio::test]
async fn test_quota() {
    let asset: AssetManager = Bybit::new(api_key(), secret());
    let quota = asset
        .apply_for_quota(
            "ETH",
            "USDT",
            "ETH",
            0.95.to_string(),
            QuotaAccountType::Funding,
        )
        .await
        .unwrap();

    tracing::info!("{:?}", quota);
}

#[tokio::test]
async fn test_query_all_coins() {
    let asset: AssetManager = Bybit::new(api_key(), secret());
    let coins = asset
        .query_all_coins(AccountType::Funding, None)
        .await
        .unwrap();

    tracing::info!("{:?}", coins);
}
