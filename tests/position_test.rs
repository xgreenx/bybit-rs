use bybit::{
    api::*,
    enable_tracing,
    model::{
        Category,
        LeverageRequest,
        PositionRequest,
    },
    position::PositionManager,
    test_utils::{
        api_key,
        secret,
    },
};
use tokio::test;

enable_tracing!();

#[test]
async fn position_info() {
    let position: PositionManager = Bybit::new(api_key(), secret());
    let request =
        PositionRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None);
    match position.get_info(request).await {
        Ok(data) => tracing::info!("{:?}", data),
        Err(e) => tracing::error!("{:?}", e),
    }
}

#[test]
async fn set_leverage() {
    let position: PositionManager = Bybit::new(api_key(), secret());
    let request = LeverageRequest::new(Category::Linear, "BTCUSDT", 10);
    match position.set_leverage(request).await {
        Ok(data) => tracing::info!("{:?}", data),
        Err(e) => tracing::error!("{:?}", e),
    }
}
