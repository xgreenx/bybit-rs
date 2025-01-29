use bybit::{
    api::*,
    enable_tracing,
    model::*,
    test_utils::{
        api_key,
        secret,
    },
    trade::*,
};
use tokio;

enable_tracing!();

#[tokio::test]
async fn test_trade() {
    let trade: Trader = Bybit::new(api_key(), secret());
    let order = trade
        .place_futures_limit_order(
            Category::Linear,
            "MATICUSDT",
            Side::Buy,
            100.0,
            0.7500,
            0,
        )
        .await;
    tracing::info!("{:#?}", order);
}

#[tokio::test]
async fn test_order_history() {
    let trade: Trader = Bybit::new(api_key(), secret());
    let data: OrderHistoryRequest = OrderHistoryRequest::new(
        Category::Linear,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let order_history = trade.get_order_history(data).await;
    tracing::info!("{:#?}", order_history);
}

#[tokio::test]
async fn test_trade_history() {
    let trade: Trader = Bybit::new(api_key(), secret());
    let data: TradeHistoryRequest = TradeHistoryRequest::new(
        Category::Linear,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let trade_history = trade.get_trade_history(data).await;
    if let Ok(data) = trade_history {
        tracing::info!("{:#?}", data.result.list);
    }
}

#[tokio::test]
async fn test_batch() {
    let trade: Trader = Bybit::new(api_key(), secret());
    let request = vec![
        OrderRequest {
            symbol: "MATICUSDT".into(),
            side: Side::Buy,
            qty: 100.0,
            order_type: OrderType::Market,
            ..Default::default()
        },
        OrderRequest {
            symbol: "BTCUSDT".into(),
            side: Side::Buy,
            qty: 100.0,
            order_type: OrderType::Market,
            ..Default::default()
        },
    ];
    let data: BatchPlaceRequest = BatchPlaceRequest::new(Category::Linear, request);
    let batch = trade.batch_place_order(data).await;
    tracing::info!("{:#?}", batch);
}

#[tokio::test]
async fn test_open_closed_orders() {
    let trade: Trader = Bybit::new(api_key(), secret());
    let request = OpenOrdersRequest::custom(
        Category::Spot,
        "ETHUSDT",
        None,
        None,
        None,
        None,
        0,
        None,
        None,
    );
    let result = trade.get_open_orders(request).await.unwrap();
    tracing::info!("{:#?}", result);
}
