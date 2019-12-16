use crate::{generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    let pageButtonStyle = style!{
        St::Width => "10%",
        St::Background => "#d9e2df",
        St::BorderRadius => "3px",
        St::MarginLeft => "1%",
        St::MarginTop => "15px"
    };
    div![
        style!{
            St::FontSize => "18px",
            St::Width => "50%",
            St::TextAlign => "center",
            St::MarginTop => "15px",
            St::MarginLeft => "25%",
            St::MarginBottom => "100px"
        },
        span![
            format!("Total : 12")
        ],
        button![
            &pageButtonStyle,
            simple_ev(Ev::Click, Msg::FirstPage),
            "First"
        ],
        button![
            &pageButtonStyle,
            simple_ev(Ev::Click, Msg::LastPage),
            "Last"
        ],
        button![
            &pageButtonStyle,
            simple_ev(Ev::Click, Msg::NextPage),
            "Next"
        ],
        button![
            &pageButtonStyle,
            simple_ev(Ev::Click, Msg::TailPage),
            "Tail"
        ],
        input![
            style!{
                St::Width => "10%",
                St::MarginLeft => "1%"
            },
            attrs![
                At::Type => "number"
            ]
        ],
        button![
            &pageButtonStyle,
            simple_ev(Ev::Click, Msg::LookPage),
            "Look"
        ]

    ]
}
