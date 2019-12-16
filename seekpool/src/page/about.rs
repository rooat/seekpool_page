use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view() -> impl View<Msg> {
    div![
        class![
            C.flex_grow,
        ],
        // Photo section
        section![
            class![
                C.w_screen,
                C.h_690px,
                C.bg_blue_10,
                // sm__
                C.sm__h_790px,
                // lg__
                C.lg__h_1420px,
            ],
            div![
                "about ..."
            ]

        ],
    ]
}
