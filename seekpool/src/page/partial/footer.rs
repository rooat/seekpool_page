use crate::{generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    footer![
        class![
            C.h_16,
            C.shadow_2xl_above,
            C.flex,
            C.justify_center,
            // sm__
            C.sm__h_24,
        ],
        div![
            class![
                C.w_xs,
                C.h_full,
                C.px_5,
                C.flex,
                C.justify_between,
                C.items_center,
                // sm__
                C.sm__w_132
            ],
            div![
                class![
                    C.flex,
                    C.text_20
                ],
                "seekchain@seekchain.org"
            ],
            div![
                class![C.cursor_pointer, C.h_full, C.flex, C.items_center,],
                simple_ev(Ev::Click, Msg::ScrollToTop),
                img![
                    class![
                        C.mt_1, C.w_12, // sm__
                        C.sm__w_16
                    ],
                    attrs! {
                        At::Src => image_src("top.svg")
                    }
                ],
            ]
        ]
    ]
}
