use bybit::{
    api::*,
    config::*,
    enable_tracing,
    market::*,
    model::{
        Category,
        FundingHistoryRequest,
        HistoricalVolatilityRequest,
        InstrumentRequest,
        KlineRequest,
        OpenInterestRequest,
        OrderbookRequest,
        RecentTradesRequest,
        RiskLimitRequest,
    },
    test_utils::{
        api_key,
        secret,
    },
};
use tokio::{
    self,
    time::{
        Duration,
        Instant,
    },
};

enable_tracing!();

#[tokio::test]
async fn test_kline() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let request = KlineRequest::new(
        Some(Category::Linear),
        "MATICUSDT",
        "60",
        Some("010124"),
        Some("050224"),
        None,
    );
    let premium = market.get_klines(request).await;
    if let Ok(data) = premium {
        tracing::info!("{:#?}", data.result.list);
    }
}

#[tokio::test]
async fn test_instrument() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let request =
        InstrumentRequest::new(Category::Linear, Some("APTUSDT"), None, None, None);
    let instrument = market.get_futures_instrument_info(request.clone()).await;
    if let Ok(data) = instrument {
        tracing::info!("{:#?}", data.result.list[0]);
    }
    let spot_instrument = market.get_spot_instrument_info(request).await;
    if let Ok(data) = spot_instrument {
        tracing::info!("{:#?}", data.result.list[0]);
    }
}

#[tokio::test]
async fn test_market() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let end_minutes = Duration::from_secs(5);
    let request = OrderbookRequest::new("ETHUSDT", Category::Spot, Some(1));
    let start = Instant::now();
    while Instant::now() - start < end_minutes {
        let data = market.get_depth(request.clone()).await.unwrap();
        let order_book = data.result;
        let mid_price = (order_book.asks[0].price + order_book.bids[0].price) / 2.0;
        let imbalance = (order_book.bids[0].qty - order_book.asks[0].qty)
            / (order_book.asks[0].qty + order_book.bids[0].qty);
        let fees = fee_percent(mid_price, 0.04);
        let spread = order_book.asks[0].price - order_book.bids[0].price;
        let arb = spread - fees;
        tracing::info!(
            "{:#?} , Spread: {:.5} Arb: {} Imb: {:.4}",
            order_book,
            spread,
            if arb > fee_percent(mid_price, 0.02) {
                arb
            } else {
                0.0
            },
            imbalance
        );
    }
}

fn fee_percent(value: f64, percent: f64) -> f64 {
    (percent / 100.0) * value
}

#[tokio::test]
async fn test_ticker() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = "APTUSDT";
    let ticker = market.get_futures_tickers(Some(symbol)).await;
    if let Ok(data) = ticker {
        tracing::info!("{:#?}", data.result.list);
    }
    let spot_ticker = market.get_spot_tickers(Some(symbol)).await;
    if let Ok(data) = spot_ticker {
        tracing::info!("{:#?}", data.result.list);
    }
}

#[tokio::test]
async fn test_recent_trades() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let request = RecentTradesRequest::new(Category::Linear, Some("POLUSDT"), None, None);
    let trades = market.get_recent_trades(request).await;
    if let Ok(data) = trades {
        tracing::info!("{:#?}", data.result.list);
    }
}

#[tokio::test]
async fn test_funding_rate() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = "BTCUSDT";
    let request = FundingHistoryRequest::new(Category::Linear, symbol, None, None, None);
    let funding_rate = market.get_funding_history(request).await;
    if let Ok(data) = funding_rate {
        tracing::info!("{:#?}", data.result.list.last().unwrap());
    }
}

#[tokio::test]
async fn test_open_interest() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let request =
        OpenInterestRequest::new(Category::Linear, "MATICUSDT", "4h", None, None, None);
    let open_interest = market.get_open_interest(request).await;
    if let Ok(data) = open_interest {
        tracing::info!("{:#?}", data.result.list.last().unwrap());
    }
}

#[tokio::test]
async fn test_historical_volatility() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = "ETH";
    let request: HistoricalVolatilityRequest<'_> =
        HistoricalVolatilityRequest::new(Some(symbol), None, None, None);
    let historical_volatility = market.get_historical_volatility(request).await;
    if let Ok(data) = historical_volatility {
        tracing::info!("{:#?}", data.result);
    }
}

#[tokio::test]
async fn test_insurance() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = Some("BTC");
    let insurance = market.get_insurance(symbol).await;
    if let Ok(data) = insurance {
        tracing::info!("{:#?}", data.result);
    }
}

#[tokio::test]
async fn test_risk_limit() {
    let market: MarketData =
        Bybit::new_with_config(&Config::default().set_recv_window(1000), None, None);
    let symbol = "MATICUSDT";
    let request: RiskLimitRequest<'_> =
        RiskLimitRequest::new(Category::Linear, Some(symbol));
    let risk_limit = market.get_risk_limit(request).await;
    if let Ok(data) = risk_limit {
        tracing::info!("{:#?}", data.result);
    }
}

#[tokio::test]
async fn test_delivery_price() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = "BTCUSDT";
    let delivery_price = market
        .get_delivery_price(Category::Option, Some(symbol), None, None)
        .await;
    if let Ok(data) = delivery_price {
        tracing::info!("{:#?}", data.result);
    }
}

#[tokio::test]
async fn test_longshort_ratio() {
    let market: MarketData = Bybit::new(api_key(), secret());
    let symbol = "BTCUSDT";
    let longshort_ratio = market
        .get_longshort_ratio(Category::Linear, symbol, "4h", None)
        .await;
    if let Ok(data) = longshort_ratio {
        tracing::info!("{:#?}", data.result);
    }
}
