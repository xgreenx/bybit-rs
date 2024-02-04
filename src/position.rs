use std::collections::BTreeMap;

use serde_json::Value;

use crate::api::{Position, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::*;
use crate::util::{build_json_request, build_request, date_to_milliseconds};

#[derive(Clone)]
pub struct PositionManager {
    pub client: Client,
    pub recv_window: u64,
}

impl PositionManager {
        /// Asynchronously retrieves information about a position based on the provided request.
        ///
        /// # Arguments
        ///
        /// * `req` - The position request containing the category, symbol, base coin, settle coin, and limit.
        ///
        /// # Returns
        ///
        /// A `Result` containing a vector of `PositionInfo` if the operation is successful, or an error if it fails.
        ///
        /// # Example
        ///
        /// ```
        /// use crate::model::{PositionRequest, Category};
        /// use crate::errors::Result;
        /// use crate::api::PositionInfo;
        /// use my_module::PositionManager;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<()> {
        ///     let position_manager = PositionManager::new();
        ///     let request = PositionRequest::new(Category::Linear, Some("symbol"), Some("base_coin"), Some("settle_coin"), Some(10));
        ///     let position_info = position_manager.get_info(request).await?;
        ///     Ok(())
        /// }
        /// ```
    pub async fn get_info<'a>(&self, req: PositionRequest<'a>) -> Result<Vec<PositionInfo>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.base_coin {
            parameters.insert("baseCoin".into(), v.into());
        }
        if let Some(v) = req.settle_coin {
            parameters.insert("settleCoin".into(), v.into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.to_string());
        }
        let request = build_request(&parameters);
        let response: InfoResponse = self
            .client
            .get_signed(
                API::Position(Position::Information),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result.list)
    }

    pub async fn set_leverage<'a>(&self, req: LeverageRequest<'a>) -> Result<LeverageResponse> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("buyLeverage".into(), req.leverage.to_string());
        parameters.insert("sellLeverage".into(), req.leverage.to_string());
        let request = build_json_request(&parameters);
        let response: LeverageResponse = self
            .client
            .post_signed(
                API::Position(Position::SetLeverage),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn set_margin_mode<'a>(
        &self,
        req: ChangeMarginRequest<'a>,
    ) -> Result<ChangeMarginResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into(),
        );
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert(
            "tradeMode".into(),
            req.trade_mode.into(),
        );
        let request = build_json_request(&parameters);
        let response: ChangeMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchIsolated),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn set_position_mode<'a>(
        &self,
        req: MarginModeRequest<'a>,
    ) -> Result<MarginModeResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into(),
        );
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.coin {
            parameters.insert("coin".into(), v.into());
        }
        parameters.insert("mode".into(), req.mode.into());
        let request = build_json_request(&parameters);
        let response: MarginModeResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchMode),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn set_risk_limit<'a>(&self, req: SetRiskLimit<'a>) -> Result<SetRiskLimitResult> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("riskId".into(), req.risk_id.into());
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: SetRiskLimitResponse = self
            .client
            .post_signed(
                API::Position(Position::SetRiskLimit),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn set_trading_stop<'a>(
        &self,
        req: TradingStopRequest<'a>,
    ) -> Result<TradingStopResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into(),
        );
        parameters.insert("symbol".into(),req.symbol.into());
        if let Some(v) = req.take_profit {
            parameters.insert("takeProfit".into(), v.into());
        }
        if let Some(v) = req.stop_loss {
            parameters.insert("stopLoss".into(), v.into());
        }
        if let Some(v) = req.tp_trigger_by {
            parameters.insert("tpTriggerBy".into(), v.into());
        }
        if let Some(v) = req.sl_trigger_by {
            parameters.insert("slTriggerBy".into(), v.into());
        }
        if let Some(v) = req.tpsl_mode {
            parameters.insert("tpslMode".into(), v.into());
        }
        if let Some(v) = req.tp_order_type {
            parameters.insert("tpOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.sl_order_type {
            parameters.insert("slOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.tp_size {
            parameters.insert("tpSize".into(), v.into());
        }
        if let Some(v) = req.sl_size {
            parameters.insert("slSize".into(), v.into());
        }
        if let Some(v) = req.tp_limit_price {
            parameters.insert("tpLimitPrice".into(), v.into());
        }
        if let Some(v) = req.sl_limit_price {
            parameters.insert("slLimitPrice".into(), v.into());
        }
        parameters.insert(
            "positionIdx".into(),
            req.position_idx.into());
        let request = build_json_request(&parameters);
        let response: TradingStopResponse = self
            .client
            .post_signed(
                API::Position(Position::SetTradingStop),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn set_add_margin<'a>(&self, req: AddMarginRequest<'a>) -> Result<AddMarginResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into()
        );
        parameters.insert("symbol".into(), req.symbol.into());
        if req.auto_add {
            parameters.insert("autoAddMargin".into(), 1.into());
        } else {
            parameters.insert("autoAddMargin".into(), 0.into());
        }
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: AddMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SetAutoaddMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn add_or_reduce_margin<'a>(
        &self,
        req: AddReduceMarginRequest<'a>,
    ) -> Result<AddReduceMarginResult> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("margin".into(), req.margin.into());
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: AddReduceMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::AddorReduceMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn get_closed_pnl<'a>(&self, req: ClosedPnlRequest<'a>) -> Result<ClosedPnlResult> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }

        if let Some(start_str) = req.start_time.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| start_millis.to_string().into());
        }
        if let Some(end_str) = req.end_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string().into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.into());
        }
        let request = build_request(&parameters);
        let response: ClosedPnlResponse = self
            .client
            .get_signed(
                API::Position(Position::ClosedPnl),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }
    
}
