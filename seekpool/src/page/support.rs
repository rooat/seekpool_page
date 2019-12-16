use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        class![
            C.flex_grow,
        ],
        div![
            style!{
                St::Position => "fixed",
                St::Left => "30%",
                St::Width => "40%",
                St::TextAlign => "center",
                St::MarginTop => "120px",
                St::FontSize => "20px"
            },
            "More miner detail  look at ",
            a![
                style!{
                    St::Color => "red"
                },
                "click here",
                attrs! {
                    At::Target => "_blank",
                    At::Href => "http://git.etznumberone.com/seek-chain/seek-chain-release/blob/master/seek-chain-api-changing.md"
                },
            ]
        ]
    ]
}
