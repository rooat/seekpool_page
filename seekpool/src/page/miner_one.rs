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
                    b![
                        "Miner : "
                    ],
                    a![
                        style!{
                            St::Color => "#4c7cea"
                        },
                        attrs![
                            At::Href => format!("{}/#/eaaddress?address={}",model.config.url.explorer,"0xe07a8a8153D1C159dfbaC0fEfE9812C7B5583B8d"),
                            At::Target => "_blank"
                        ],
                        "0xe07a8a8153D1C159dfbaC0fEfE9812C7B5583B8d"
                    ]

                ],
                p![
                    b![
                        "Author : "
                    ],
                    "0xe07a8a8153D1C159dfbaC0fEfE9812C7B5583B8d"
                ],
                p![
                    b![
                        "PID : "
                    ],
                    "2332434345"
                ],

                div![
                    class![
                        C.sm__flex
                    ],
                    div![
                        p![
                            b![
                                "Level : "
                            ],
                            "9"
                        ],
                        p![
                            b![
                                "Rate : "
                            ],
                            "12%"
                        ],
                        p![
                            b![
                                "BurntCoin : "
                            ],
                            "232323"
                        ],

                    ],
                    div![
                        style!{
                            St::MarginLeft => "20%"
                        },
                        p![
                            b![
                                "BlockCount : "
                            ],
                            "1233"
                        ],
                        p![
                            b![
                                "TotalBenefit : "
                            ],
                            "1233244"
                        ],
                        p![
                            b![
                                "AverageBenefit : "
                            ],
                            "232323"
                        ]
                    ]
                ],
                p![
                    b![
                        "Ranking : "
                    ],
                    "1"
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
