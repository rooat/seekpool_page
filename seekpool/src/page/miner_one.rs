use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
    Model , Page, ScrollHistory,
    Visibility::{self, *},U256,FromStr,
    ResState::{self, *}
};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> impl View<Msg> {

    vec![

        div![
            class![
                C.flex_grow,
            ],
            style!{
                St::Width => "70%",
                St::MarginLeft => "15%",
                St::MarginTop => "40px",
                St::MarginBottom => "100px"
            },
            div![
                style!{
                    St::MarginTop => "100px",
                    St::FontSize => "20px"
                },

                p![
                   format!("Miner : 0xdgdgdgdg")
                ],
                p![
                   format!("Author : 0xdgdgdgdg")
                ],
                p![
                   format!("Pid : 123232344")
                ],

                div![
                    class![
                        C.sm__flex
                    ],
                    div![
                        p![
                            format!("Level : 9")
                        ],
                        p![
                            format!("Rate : 12%")
                        ],
                        p![
                            format!("BurntCoin : 1222222")
                        ],

                    ],
                    div![
                        style!{
                            St::MarginLeft => "20%"
                        },
                        p![
                           format!("BlockCount : 0xdgdgdgdg")
                        ],
                        p![
                           format!("TotalBenefit : 12222")
                        ],
                        p![
                           format!("EvBenefit: 33333")
                        ]
                    ]
                ],
                p![
                    format!("Current ranking : 1")
                ]

            ],
            div![
                style!{
                    St::MarginTop => "20px",
                    St::Height => "370px",
                    St::MaxWidth => "100%",
                },
                attrs![
                    At::Id => "chartContainer_cost",
                ],
            ],
            div![
                style!{
                    St::MarginTop => "20px",
                    St::Height => "370px",
                    St::MaxWidth => "100%",
                },
                attrs![
                    At::Id => "chartContainer",
                ],
            ],
        ],
    ]
}
