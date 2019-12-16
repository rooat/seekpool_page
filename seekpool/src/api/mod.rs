use crate::{
    Msg, Model,Page,TITLE_SUFFIX,document,utils,Visibility,
    NewRegister,ResState,Orders,Future,RequestBody,Request,REGIST_URL,Method
};

pub fn edit_change(model: &mut Model ,input_text : &str){
    if input_text.len() ==42 && input_text.starts_with("0x") {
        model.input_text = Some(input_text.into())
    }else {
        model.input_text = None
    }
}

pub fn update_title(model: &mut Model){
    let title = match model.page {
        Page::Home => {
            TITLE_SUFFIX.to_owned()
        },
        Page::Miners => {
            format!("Miners - {}",TITLE_SUFFIX)
        },
        Page::MinerOne => {
            format!("MinerOne - {}",TITLE_SUFFIX)
        },
        Page::Payments => {
            format!("Payments - {}",TITLE_SUFFIX)
        },
        Page::Support => {
            format!("Support - {}",TITLE_SUFFIX)
        },
        Page::About => format!("About - {}", TITLE_SUFFIX),
        Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
    };
    document().set_title(&title);
}

pub fn fetch_miner(model : &mut Model ,orders: &mut impl Orders<Msg>){
    let input_option: Option<String> = model.input_text.clone();
    match input_option {
        None => log!("input null"),
        Some(t) => {
            if t.to_owned().len() ==42 {
                let url = &model.config.url.host;
                orders.skip().perform_cmd(utils::fetchMinerData(format!("{}/api/miners/info?address={}",url, t)));
                model.show_valibale = Visibility::IsShow;
                model.res_state = ResState::InitState;
                log!(format!("show_valibale:{:?},res_state:{:?}",model.show_valibale,model.res_state));
            }else{
                log!("address invalid ");
            }
        }
    }
}

pub fn regist_miner(model : &mut Model){
    model.new_register = NewRegister::new("0xcaaf10244e0f891a2c4f066f3d137914b47f1dce".into(),"0xe54a3edf8aa5f82123fb54ef165b16cc32017801".into());
//            orders.skip().perform_cmd(registMiner(model));
    model.res_state = ResState::SuccessState;
}

