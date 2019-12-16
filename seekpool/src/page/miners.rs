use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
     Model , Page, ScrollHistory,to_page,
    Visibility::{self, *},U256,FromStr,
    ResState::{self, *},utils
};
use seed::{prelude::*, *};


pub fn view(model: &Model) -> impl View<Msg> {
    let is_show = model.show_valibale == IsShow;
    let burnt_coin = U256::from_str(*&model.minerData.burnt_coin.trim_start_matches("0x")).unwrap();
    let reward_level = U256::from_str(*&model.minerData.reward_level.trim_start_matches("0x")).unwrap();
    let max_storage = U256::from_str(*&model.minerData.max_storage.trim_start_matches("0x")).unwrap();

    let burnt_coin = utils::shortBalance(format!("{}",burnt_coin));

    let mut listVec = vec![];
    let mut inds = 0;
    for miner in &model.miner_list{
        let mut tr_style = style!{
            St::Background => "#f7fbf4"
        };
        if inds % 2 ==0{
            tr_style = style!{
                St::Background => "#eeefee"
            };
        }
        let miner_add = &miner.miner[0..9];
        let listV = div![
            tr_style,
            class![
                C.table_row,
                C.sm__text_15,
                C.md__text_18,
                C.text_center,
                C.sm__h_8,

            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
            div![
                class![
                    C.table_cell,
                    C.md__leading_loose
                ],
                miner_add
            ],
        ];
        listVec.push(listV);
        inds += 1;
    }


    let pageButtonStyle = style!{
        St::Width => "10%",
        St::Background => "#d9e2df",
        St::BorderRadius => "3px",
        St::MarginLeft => "1%",
        St::MarginTop => "15px"
    };
    log!(format!("Response data2: {:#?}", burnt_coin));
//    let s = &model.minerData.burnt_coin.as_bytes();

    vec![
        if is_show {
            div![
                style!{
                    St::Position => "fixed",
                    St::Top => "200px",
                    St::ZIndex => "17",
                    St::Width => "50%",
                    St::Left => "25%",
                    St::BorderRadius => "7px",
                    St::Background => "white",
                    St::PaddingTop => "30px",
                    St::PaddingLeft => "40px",
                    St::FontSize => "0.9rem",
                },
                match model.res_state {
                    InitState =>div![
                        p![
                            format!("Miner : {}",model.minerData.owner)
                        ],
                        p![
                            format!("Inviter : {}",model.minerData.inviter)
                        ],
                        p![
                            format!("Author : {}",model.minerData.author)
                        ],
                        p![

                            format!("BurntCoin : {}",burnt_coin)
                        ],
                        p![
                            format!("RewardLevel : {}",reward_level)
                        ],
                        p![
                            format!("MaxStorage : {}",max_storage)
                        ],
                    ],
                    SuccessState => div![
                        p![
                            format!("Miner : {}",model.new_register.owner)
                        ],
                        p![
                            format!("Author : {}",model.new_register.author)
                        ],
                        p![
                            format!("Pid : {}",model.new_register.pid)
                        ]
                    ],
                    FailureState => div![
                        p![
                            "failure"
                        ]
                    ]
                },

                div![
                    style!{
                        St::MarginTop => "20px",
                        St::MarginBottom => "20px",
                        St::MarginLeft => "50%"
                    },
                    button![
                        style!{
                            St::Background => "#f3f0f0",
                            St::Padding => "7px",
                            St::BorderRadius => "7px",
                            St::Color => "#c15f58"
                        },
                        attrs![
                           At::OnClick => "close_tips()"
                        ],
                        simple_ev(Ev::Click, Msg::IsHide),
                        "Close"
                    ],
                    button![
                        style!{
                            St::MarginLeft => "40px",
                            St::Background => "#f3f0f0",
                            St::Padding => "7px",
                            St::BorderRadius => "7px",
                            St::Color => "#0b9484"
                        },
                        simple_ev(Ev::Click, Msg::Regist),
                        "Confirm"
                    ]
                ],
                attrs![
                    At::Id => "dataid"
                ]
            ]
        }else {
            empty![]
        },
        div![
            class![
                C.flex_grow,
            ],
            style!{
                St::MarginLeft => "15%",
                St::Width => "70%"
            },
            //To be miner
            div![
                class![
                    C.sm__flex
                ],
                style!{
                    St::MarginTop => "150px",

                    St::Width => "100%",
                },
                span![
                    style!{
                        St::Display => "inline-block",
                        St::Width => "80%",
                        St::Border => "1px solid grey",
                        St::BorderRadius => "7px"
                    },
                    input![
                        style!{
                           St::BorderRadius => "7px",
                           St::Height => "60px",
                           St::Width => "100%",
                           St::Background => "#f1f5f8",
                           St::PaddingLeft => "5px",
                           St::Outline => "none",
                           St::FontSize => "28px"
                        },
                        attrs![
                            At::Type => "text",
                            At::Placeholder => "Input address to regist miner",
                            At::Id => "inputaddress"
                        ],
                        input_ev(Ev::Input, Msg::EditChange)
                    ],
                ],
                button![
                    style!{
                        St::Width => "20%",
                        St::Background => "red"
                        St::BorderRadius => "7px",
                        St::Color =>"white",
                        St::Background => "#1cb3a1",
                        St::FontSize => "20px"
                    },
                    attrs![
                        At::OnClick => "invalid_input()",
                    ],
                    simple_ev(Ev::Click, Msg::FetchMiner),

                    "Join Us"
                ]
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
            //miner list
            div![
                style!{
                    St::Width => "100%",
                },
                div![
                    class![
                        C.sm__my_6
                    ],
                    div![
                        class![
                            "table w-full"
                        ],
                        div![
                            class![
                                C.table_row
                                C.text_center
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                            div![
                                class![
                                    C.table_cell
                                ],
                                "miner"
                            ],
                        ],
                        listVec
                    ],

                ],
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
                        format!("Total : {}",model.page_size.total_count)
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

            ],

        ],
    ]
}
