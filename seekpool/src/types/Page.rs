use crate::{
    Url
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    Miners,
    MinerOne,
    Payments,
    Support,
    About,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Miners => "/miners",
            Self::MinerOne => "/miner_one",
            Self::Payments => "/payments",
            Self::Support => "/support",
            Self::About => "/about",
            Self::NotFound => "/404",
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        match url.path.first().map(String::as_str) {
            None | Some("") => Self::Home,
            Some("about") => Self::About,
            Some("miner_one") => Self::MinerOne,
            Some("miners") => Self::Miners,
            Some("payments") => Self::Payments,
            Some("support") => Self::Support,
            _ => Self::NotFound,
        }
    }
}