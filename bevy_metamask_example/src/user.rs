use bevy::prelude::*;
use bevy_metamask::{
    GetBalance, MetamaskRequest, MetamaskRequestEvent, MetamaskResponse, RequestAccounts,
};

#[derive(Default, Resource)]
pub struct User {
    pub address: Option<String>,
    pub balance: Option<String>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let address: &str = match &self.address {
            Some(addr) => addr,
            None => "unknown",
        };

        let balance: &str = match &self.balance {
            Some(bal) => bal,
            None => "unknown",
        };

        write!(f, "User ({address})\nBalance ({balance})")
    }
}

pub enum UserStatusRequest {
    Address,
    Balance(String),
}

impl MetamaskRequestEvent for UserStatusRequest {
    type ResponseEvent = UserStatusResponse;

    fn to_req(&self) -> MetamaskRequest {
        match self {
            UserStatusRequest::Address => MetamaskRequest::RequestAccounts(RequestAccounts),
            UserStatusRequest::Balance(address) => MetamaskRequest::GetBalance(GetBalance {
                data: address.clone(),
            }),
        }
    }

    fn from_resp(resp: MetamaskResponse) -> Self::ResponseEvent {
        match resp {
            MetamaskResponse::RequestAccounts(mut addresses) => {
                UserStatusResponse::Address(addresses.remove(0))
            }
            MetamaskResponse::GetBalance(balance) => UserStatusResponse::Balance(balance),
            _ => unreachable!(),
        }
    }
}

pub enum UserStatusResponse {
    Address(String),
    Balance(String),
}
