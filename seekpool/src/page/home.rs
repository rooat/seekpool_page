use crate::{
    generated::css_classes::C,Model, image_src, Msg, Page, MAIL_TO_HELLWEB,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};
use std::str;

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> impl View<Msg> {
    let table_width = style!{
        St::Width => "25%"
    };
    let table_width_box = style!{
        St::MarginBottom => "40px",
        St::MarginLeft => "50px"
    };
    let dl_margin_left = style!{
        St::MarginLeft => "20px"
    };


    let m_and_p_style = match model.menu_visibility {
        Visible => style!{
            St::Background => "#49d28e",
            St::Width => "100%",
            St::BorderRadius => "7px",
            St::Color => "white",
            St::TextAlign => "center",
            St::Height => "60px",
            St::LineHeight => "60px"
        },
        _ => style!{
            St::Background => "#49d28e",
            St::Width => "20%",
            St::BorderRadius => "7px",
            St::Color => "white",
            St::TextAlign => "center",
            St::Height => "60px",
            St::LineHeight => "60px"
        }
    };

    div![
        class![
            C.flex_grow,
            C.container,
            C.mx_auto,
            C.sm__mb_24,
        ],
        div![
            class![
                C.lg__flex,
                "h-16",
            ],
            style!{
                St::Border => "1px solid grey",
                St::BorderRadius => "7px",
                St::MarginTop => "120px"
            },
            div![
                class![
                    "h-16"
                ],
                style!{
                    St::Width => "80%",
                    St::BorderRadius => "7px",
                },
                input![
                    style!{
                       St::BorderRadius => "7px",
                       St::Height => "30px",
                       St::Width => "100%",
                       St::Background => "#f1f5f8",
                       St::PaddingLeft => "5px",
                       St::Outline => "none",
                       St::FontSize => "28px"
                    },
                    attrs![
                        At::Type => "text",
                        At::Placeholder => "Input address to Search"
                    ]
                ],
            ],
            div![
                class![
                    "h-16",
                    "md:w-48",
                ],
                style!{
                    St::Background => "#49d28e",
                    St::Width => "20%",
                    St::BorderRadius => "7px",
                    St::Color => "white",
                    St::TextAlign => "center",
                    St::LineHeight => "60px"
                },
                a![
                    attrs! {
                        At::Href => Page::MinerOne.to_href(),
                        At::OnClick => "miner_one()"
                    },
                    simple_ev(Ev::Click, Msg::ScrollToTop),
                    simple_ev(Ev::Click, Msg::HideMenu),
                    "Search"
                ]
            ],

        ],
        div![
            class![
                C.sm__flex,
                C.sm__text_15,
                C.md__text_18

            ],
            style!{
                St::MarginTop => "80px",
                St::MarginBottom => "50px"
                St::Width => "100%",

                St::BorderRadius => "7px",
                St::Padding => "20px",
                St::BoxShadow => "-1px 2px 20px 0px rgba(0, 0, 0, 0.25), -1px -4px 5px 0px rgba(0, 0, 0, 0.25)"
            },
            div![
                style!{
                    St::Width => "40%",
                    St::TextAlign => "center",
                    St::LineHeight => "50px"
                },
                dl![
                    class![
                        "sm:flex-wrap"
                    ],
                    dd![
                        "当前区块高度"
                    ],
                    p![
                        "23333"
                    ],

                ]
            ],
            div![
                class![
                    C.sm__flex
                ],
                style!{
                    St::Width => "100%"
                },
                div![
                    style!{
                        St::Width => "100%",
                        St::TextAlign => "left"
                    },
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![

                            attrs![
                                At::Src => image_src("icon_1.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "全网容量"
                            ],
                            p![
                                "233.33PB"
                            ]
                        ]
                    ],
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![
                            attrs![
                                At::Src => image_src("icon_2.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "在线矿工"
                            ],
                            p![
                                "633"
                            ]
                        ]
                    ],
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![
                            attrs![
                                At::Src => image_src("icon_3.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "上个挖到的区块"
                            ],
                            p![
                                "23333"
                            ]
                        ]
                    ],
                ],
                div![
                    style!{
                        St::Width => "100%",
                        St::TextAlign => "left"
                    },
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![
                            attrs![
                                At::Src => image_src("icon_4.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "矿池容量"
                            ],
                            p![
                                "233.33PB"
                            ]
                        ]
                    ],
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![
                            attrs![
                                At::Src => image_src("icon_5.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "在线矿机"
                            ],
                            p![
                                "23344"
                            ]
                        ]
                    ],
                    div![
                        &table_width_box,
                        class![
                            C.sm__flex,
                        ],
                        img![
                            attrs![
                                At::Src => image_src("icon_6.png")
                            ]
                        ],
                        dl![
                            &dl_margin_left,
                            dd![
                                "24小时出块数"
                            ],
                            p![
                                "23388"
                            ]
                        ]
                    ],
                ],
            ]
        ],
        div![
            style!{
                St::MarginTop => "20px",
                St::Height => "370px",
                St::MaxWidth => "100%",
            },
            attrs![
                At::Id => "chartContainer_home",
            ],
        ],

    ]
}
