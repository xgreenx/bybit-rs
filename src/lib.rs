#![deny(unused_crate_dependencies)]

pub mod account;
pub mod api;
pub mod asset;
pub mod client;
pub mod config;
pub mod errors;
pub mod general;
pub mod market;
pub mod model;
pub mod position;
pub mod trade;
pub mod util;
pub mod ws;

pub mod test_utils {
    use ctor::ctor;
    use std::{
        env::var,
        fs,
    };
    use tracing_subscriber::{
        fmt::format,
        EnvFilter,
    };

    #[ctor]
    pub static TRACE: () = {
        if let Ok(v) = var("TRACE") {
            match v.to_lowercase().as_str() {
                "1" | "true" | "on" => {
                    let _ = tracing_subscriber::FmtSubscriber::builder()
                        .with_env_filter(EnvFilter::from_default_env())
                        .try_init();
                }
                "compact" => {
                    let _ = tracing_subscriber::FmtSubscriber::builder()
                        .with_env_filter(EnvFilter::from_default_env())
                        .event_format(format().compact())
                        .try_init();
                }
                "pretty" => {
                    let _ = tracing_subscriber::FmtSubscriber::builder()
                        .with_env_filter(EnvFilter::from_default_env())
                        .event_format(format().pretty())
                        .try_init();
                }
                "log-file" => {
                    let log_path = var("TRACE_PATH").unwrap_or_else(|_| {
                        concat!(env!("CARGO_MANIFEST_DIR"), "/logs").to_string()
                    });
                    let log_file = tracing_appender::rolling::daily(log_path, "logfile");

                    let _ = tracing_subscriber::FmtSubscriber::builder()
                        .event_format(format().compact().pretty())
                        .with_ansi(false) // disabling terminal color fixes this issue: https://github.com/tokio-rs/tracing/issues/1817
                        .with_writer(log_file)
                        .try_init();
                }
                "log-show" => {
                    use tracing_subscriber::prelude::*;
                    let log_path = var("TRACE_PATH").unwrap_or_else(|_| {
                        concat!(env!("CARGO_MANIFEST_DIR"), "/logs").to_string()
                    });
                    let log_file = tracing_appender::rolling::daily(log_path, "logfile");

                    let log = tracing_subscriber::fmt::Layer::new()
                        .compact()
                        .pretty()
                        .with_ansi(false) // disabling terminal color fixes this issue: https://github.com/tokio-rs/tracing/issues/1817
                        .with_writer(log_file);

                    let subscriber = tracing_subscriber::registry()
                        .with(EnvFilter::from_default_env())
                        .with(
                            tracing_subscriber::fmt::Layer::new()
                                .with_writer(std::io::stderr),
                        )
                        .with(log);
                    let _ = subscriber.try_init();
                }
                _ => (),
            }
        }
    };

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Bybit {
        api_key: String,
        secret: String,
    }

    pub fn api_key() -> Option<String> {
        let path = var("BYBIT_JSON").ok()?;
        let json_bytes = fs::read(path).ok()?;
        let json: Bybit = serde_json::from_slice(&json_bytes).unwrap();

        Some(json.api_key)
    }

    pub fn secret() -> Option<String> {
        let path = var("BYBIT_JSON").ok()?;
        let json_bytes = fs::read(path).ok()?;
        let json: Bybit = serde_json::from_slice(&json_bytes).unwrap();

        Some(json.secret)
    }

    #[macro_export]
    macro_rules! enable_tracing {
        () => {
            static _TRACE: &$crate::test_utils::TRACE<()> = &$crate::test_utils::TRACE;
        };
    }
}
