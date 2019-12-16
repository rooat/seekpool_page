// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]

extern crate ethereum_types;
#[macro_use]
extern crate seed;
extern crate web_sys;

extern crate toml;
use std::borrow::Cow;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use futures::Future;
use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Deserialize, Serialize};
use web_sys::console;
use wasm_bindgen::JsValue;

use std::str::FromStr;
use ethereum_types::{U256, H160,Address};

mod generated;
mod page;
mod types;
mod utils;
mod api;
mod test;

use page::partial::to_page;
use types::Miner::{Miner, MinerDetail, NewRegister};
use types::ResState::ResState::{self, *};
use types::PageSize::PageSize;
use types::Config::Config;
use types::Msg::Msg;
use types::Page::Page;
use types::Visibility::Visibility;
use types::Model::Model;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{events::Listener, prelude::*, *};
use seed::fetch::FailReason::RequestError;
use crate::types::Page::Page::MinerOne;

const TITLE_SUFFIX: &str = "SeekChainPool";
const MAIL_TO_KAVIK: &str = "mailto:martin@kavik.cz?subject=Something%20for%20Martin&body=Hi!%0A%0AI%20am%20Groot.%20I%20like%20trains.";
const MAIL_TO_HELLWEB: &str =
    "mailto:martin@hellweb.app?subject=Hellweb%20-%20pain&body=Hi!%0A%0AI%20hate";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";
const REGIST_URL: &str = "http://106.13.216.136:8080/api/miners/register";



type ScrollHistory = FixedVecDeque<[i32; 3]>;



#[derive(Serialize)]
struct RequestBody {
    pub address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct  ResponseBody {
    pub miner : String,
    pub author : String,
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}


pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url))
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::EditChange(input_text) => {
            api::edit_change(model, &input_text);
        }

        Msg::RouteChanged(url) => {
            model.page = url.into();
            orders.send_msg(Msg::UpdatePageTitle);
        }

        Msg::UpdatePageTitle => {
            api::update_title(model);
        }

        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled(position) => {
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Visibility::Hidden;
        },
        Msg::IsShow => {
            model.show_valibale = Visibility::IsShow;
        },
        Msg::IsHide => {
            model.show_valibale = Visibility::Hidden;
        },

        Msg::RepositoryInfoFetched(Ok(response_data)) => {
            model.minerData = response_data;
        }

        Msg::RepositoryInfoFetched(Err(request_error)) => {
            model.minerData = MinerDetail::default();
        }

        Msg::FetchMiner => {
            api::fetch_miner(model,orders);
        }

        Msg::Regist => {
            api::regist_miner(model);
        }

        Msg::MessageSent(Ok(response_data)) => {
            model.new_register = NewRegister::new(response_data.miner,response_data.author);
            orders.skip();
        }

        Msg::MessageSent(Err(fail_reason)) => {
            orders.skip();
        }

        Msg::FirstPage => {
            model.miner_list = test::first_page_data();
        }

        Msg::LastPage => {
            model.miner_list = test::second_page_data()
        }

        Msg::NextPage => {
            model.miner_list = test::third_page_data();
        }

        Msg::TailPage => {
            model.miner_list = test::fourth_page_data();
        }

        Msg::LookPage => {
            model.miner_list = test::fifth_page_data();
        }

    }
}



pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}


pub fn window_events(_: &Model) -> Vec<Listener<Msg>> {
    vec![raw_ev(Ev::Scroll, |_| {
        let mut position = body().scroll_top();
        if position == 0 {
            position = document()
                .document_element()
                .expect("cannot get document element")
                .scroll_top()
        }
        Msg::Scrolled(position)
    })]
}

pub fn view(model: &Model) -> impl View<Msg> {
    // @TODO: Setup `prerendered` properly once https://github.com/David-OConnor/seed/issues/223 is resolved
    let prerendered = true;


    div![
        class![
            C.fade_in => !prerendered,
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view(model).els(),
            Page::Miners => page::miners::view(model).els(),
            Page::MinerOne => page::miner_one::view(model).els(),
            Page::Payments => page::payments::view(model).els(),
            Page::Support => page::support::view().els(),
            Page::About => page::about::view().els(),
            Page::NotFound => page::not_found::view().els(),
        },

        page::partial::header::view(model).els(),
        page::partial::footer::view().els(),
    ]
}

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

pub fn after_mount(
    url: Url,
    orders: &mut impl Orders<Msg>,
) -> AfterMount<Model> {

    orders.send_msg(Msg::UpdatePageTitle);

    AfterMount::new(Model::default(url)).url_handling(UrlHandling::None)
}



#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(routes)
        .build_and_start();
}
