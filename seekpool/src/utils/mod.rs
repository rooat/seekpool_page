use crate::{
    H160,Address,FromStr,toml,Cow,Future,Msg,Request,RequestBody,Method,REGIST_URL
};

pub fn calPID(address : &str) -> u64{
    let addr = Address::from_str(address.trim_start_matches("0x")).unwrap();
    let pid = addr.to_low_u64_be();
    pid
}

pub fn shortBalance(bal : String) ->i32{
    let bal = bal.parse::<u128>().expect("expect a number of string");
    let balance = bal/10_u128.pow(18);
    balance as i32
}

pub fn fetchMinerData(input_s : impl Into<Cow<'static, str>>) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(input_s)
        .fetch_json_data(Msg::RepositoryInfoFetched)
}

pub fn registMiner(input_s : impl Into<Cow<'static, str>>, address : String) -> impl Future<Item = Msg, Error = Msg> {
    let message = RequestBody {
        address: address,
    };
    Request::new(input_s)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::MessageSent)
}