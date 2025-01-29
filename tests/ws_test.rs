use bybit::api::*;
use tokio;

use bybit::{
    enable_tracing,
    model::{
        Category,
        Subscription,
        Tickers,
        WebsocketEvents,
    },
    test_utils::{
        api_key,
        secret,
    },
    ws::Stream,
};
use tokio::{
    sync::mpsc,
    time::Instant,
};

enable_tracing!();

#[tokio::test]
async fn test_auth() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        ws.ws_wallet(tx).await.unwrap();
    });
    while let Some(data) = rx.recv().await {
        tracing::info!("{:#?}", data);
    }
}

#[tokio::test]
async fn ping() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let response = ws.ws_ping(true).await;
    tracing::info!("{:#?}", response);
}

#[tokio::test]
async fn test_order_book() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let request = Subscription {
        args: vec!["publicTrade.ADAUSDT"],
        op: "subscribe",
    };

    let response = ws
        .ws_subscribe(request, Category::Linear, |event| {
            match event {
                WebsocketEvents::TradeEvent(trade) => {
                    // Handle Trade
                    for v in trade.data {
                        tracing::info!(
                            "Volume: {:.3} USD, Timestamp: {}, Side: {} Time:{}",
                            v.volume * v.price,
                            v.timestamp / 6000,
                            v.side,
                            Instant::now().elapsed().as_nanos()
                        );
                    }
                }
                WebsocketEvents::OrderBookEvent(order_book) => {
                    tracing::info!("{:#?}", order_book.data);
                    // Handle OrderBook event
                }
                // Add additional matches for other variants of the WebsocketEvents enum
                WebsocketEvents::TickerEvent(ticker) => {
                    // Handle Ticker event
                    match ticker.data {
                        Tickers::Linear(linear_ticker) => {
                            tracing::info!("{:#?}", linear_ticker);
                        }
                        Tickers::Spot(spot_ticker) => {
                            tracing::info!("{:#?}", spot_ticker);
                        }
                    }
                }
                WebsocketEvents::KlineEvent(kline) => {
                    // Handle Kline
                    for v in kline.data {
                        tracing::info!("{:#?}", v);
                    }
                }
                WebsocketEvents::LiquidationEvent(liquidation) => {
                    // Handle Liquidation
                    tracing::info!("{:#?}", liquidation.data);
                }
                _ => {}
            };
            Ok(())
        })
        .await;
    tracing::info!("{:#?}", response);
}

#[tokio::test]
async fn test_default_orderbook() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let (tx, mut rx) = mpsc::unbounded_channel();
    let request = vec![(1, "POLUSDT")];
    tokio::spawn(async move {
        ws.ws_orderbook(request, Category::Linear, tx)
            .await
            .unwrap();
    });
    while let Some(data) = rx.recv().await {
        tracing::info!("{:#?}", data);
    }
}

#[tokio::test]
async fn test_default_trades() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let request = vec!["BTCUSDT", "MATICUSDT", "ETHUSDT", "ADAUSDT"];
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        ws.ws_trades(request, Category::Linear, tx).await.unwrap();
    });
    while let Some(data) = rx.recv().await {
        tracing::info!("{:#?}", data);
    }
}

#[tokio::test]
async fn test_default_tickers() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let request = vec!["ADAUSDT", "MATICUSDT"];
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        ws.ws_tickers(request, Category::Spot, tx).await.unwrap();
    });
    while let Some(data) = rx.recv().await {
        match data {
            Tickers::Linear(linear_ticker) => {
                tracing::info!("{:#?}", linear_ticker);
            }
            Tickers::Spot(spot_ticker) => {
                tracing::info!("{:#?}", spot_ticker);
            }
        }
    }
}

#[tokio::test]
async fn test_default_klines() {
    let ws: Stream = Bybit::new(api_key(), secret());
    let request = vec![("1", "MATICUSDT")];
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        ws.ws_klines(request, Category::Linear, tx).await.unwrap();
    });
    while let Some(data) = rx.recv().await {
        tracing::info!("{:#?}", data);
    }
}
