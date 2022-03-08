use jsonrpc_http_server::jsonrpc_core::futures_util::{future::Either, FutureExt};
use jsonrpc_http_server::jsonrpc_core::*;
use std::future::Future;
use std::time::Instant;
use tracing::{info, info_span, Instrument};
use uuid::Uuid;

use crate::extractor::meta::Meta;

#[derive(Default, Debug)]
pub struct LoggingMiddleware {}

impl Middleware<Meta> for LoggingMiddleware {
    type Future = FutureResponse;
    type CallFuture = middleware::NoopCallFuture;

    fn on_request<F, X>(&self, request: Request, meta: Meta, next: F) -> Either<Self::Future, X>
    where
        F: FnOnce(Request, Meta) -> X + Send,
        X: Future<Output = Option<Response>> + Send + 'static,
    {
        let start = Instant::now();
        let req_id = Uuid::new_v4().to_string();
        let span = info_span!("jsonrpc",request_id = %req_id);
        let _entered = span.entered();
        let span1 = info_span!("json calling");
        info!("Processing request {:?}", request);

        Either::Left(Box::pin(
            next(request, meta)
                .map(move |res| {
                    if let Some(v) = &res {
                        info!(
                            "response:{}",
                            serde_json::to_string_pretty(&v).unwrap_or_default()
                        );
                    }
                    info!(
                        "Processing took: {:.6}",
                        start.elapsed().as_micros() as f64 / 1000_000.0
                    );
                    res
                })
                .instrument(span1),
        ))
    }
}
