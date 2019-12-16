use crate::{
    Page,ScrollHistory,Url,test, Visibility,MinerDetail,NewRegister,ResState,Miner,PageSize,Config,is_in_prerendering
};

pub struct Model {
    pub page: Page,
    pub input_text : Option<String>,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub show_valibale: Visibility,
    pub in_prerendering: bool,
    pub minerData: MinerDetail,
    pub new_register : NewRegister,
    pub res_state :ResState,
    pub miner_list :Vec<Miner>,
    pub page_size : PageSize,
    pub config : Config

}
impl Model {
    pub fn default(url :Url) -> Self{
        Model {
            page: url.into(),
            scroll_history: ScrollHistory::new(),
            menu_visibility: Visibility::Hidden,
            show_valibale: Visibility::IsHide,
            in_prerendering: is_in_prerendering(),
            minerData: MinerDetail::default(),
            new_register : NewRegister::default(),
            res_state : ResState::InitState,
            miner_list : test::fifth_page_data(),
            page_size : PageSize::default(),
            input_text : None,
            config :Config::default()
        }
    }
}