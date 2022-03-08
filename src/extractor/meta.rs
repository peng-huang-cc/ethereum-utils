use jsonrpc_http_server::jsonrpc_core::*;

#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub auth: Option<String>,
}

impl Metadata for Meta {}
