use bybit::{
    account::AccountManager,
    api::*,
    enable_tracing,
    model::*,
    test_utils::{
        api_key,
        secret,
    },
};
use tokio;

enable_tracing!();

#[tokio::test]
async fn test_wallet() {
    let account: AccountManager = Bybit::new(api_key(), secret());
    let wallet = account.get_wallet_balance("UNIFIED", Some("ETH")).await;

    tracing::info!("{:?}", wallet);
}

#[tokio::test]
async fn test_fee_rate() {
    let account: AccountManager = Bybit::new(api_key(), secret());
    let wallet = account
        .get_fee_rate(Category::Linear, Some("BTCUSDT".to_string()))
        .await;

    tracing::info!("{:?}", wallet);
}

#[tokio::test]
async fn test_borrow_history() {
    let account: AccountManager = Bybit::new(api_key(), secret());
    let wallet = account.get_fee_rate(Category::Spot, None).await;

    tracing::info!("{:?}", wallet);
}
