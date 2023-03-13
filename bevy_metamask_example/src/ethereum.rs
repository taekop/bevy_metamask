use bevy::prelude::*;
use bevy_metamask::{BlockNumber, MetamaskRequest, MetamaskRequestEvent, MetamaskResponse};

#[derive(Default, Resource)]
pub struct Ethereum {
    pub block_number: Option<String>,
}

impl std::fmt::Display for Ethereum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.block_number {
            Some(block_number) => write!(f, "Ethereum (#{})", block_number),
            None => write!(f, "Ethereum (Unknown)"),
        }
    }
}

pub struct EthereumStatusRequest;

impl MetamaskRequestEvent for EthereumStatusRequest {
    type ResponseEvent = EthereumStatusResponse;

    fn to_req(&self) -> MetamaskRequest {
        MetamaskRequest::BlockNumber(BlockNumber)
    }

    fn from_resp(resp: MetamaskResponse) -> Self::ResponseEvent {
        match resp {
            MetamaskResponse::BlockNumber(block_number) => {
                EthereumStatusResponse { block_number }
            }
            _ => unreachable!(),
        }
    }
}

pub struct EthereumStatusResponse {
    pub block_number: String,
}
