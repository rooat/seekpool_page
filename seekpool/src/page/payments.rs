use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,Model
};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> impl View<Msg> {
    let pageButtonStyle = style!{
        St::Width => "10%",
        St::Background => "#d9e2df",
        St::BorderRadius => "3px",
        St::MarginLeft => "1%",
        St::MarginTop => "15px"
    };
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
        log!(format!("mineradd--{}",miner_add));
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
    div![
        class![
            C.flex_grow,
            C.container,
            C.mx_auto,
        ],
        div![
            style!{
                St::MarginTop => "120px",
                St::FontSize => "18px",
            },
            h4![
                "PaymentsTotal : 90"
            ]
        ],
        div![
            class![
                C.sm__my_6
            ],
            div![
                class![
                    C.table,
                    C.w_full
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
                format!("Total : 23")
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
    ]
}
