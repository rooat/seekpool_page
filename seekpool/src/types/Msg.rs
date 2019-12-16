use crate::{
    Url,MinerDetail,fetch, ResponseBody
};

#[derive(Clone)]
pub enum Msg {
    EditChange(String),
    RouteChanged(Url),
    UpdatePageTitle,
    ScrollToTop,
    Scrolled(i32),
    ToggleMenu,
    HideMenu,
    IsShow,
    IsHide,
    RepositoryInfoFetched(fetch::ResponseDataResult<MinerDetail>),
    FetchMiner,
    Regist,
    MessageSent(seed::fetch::ResponseDataResult<ResponseBody>),

    FirstPage,
    LastPage,
    NextPage,
    TailPage,
    LookPage,

}