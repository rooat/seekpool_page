//use crate::{
//    web3,Config
//};
//
//use std::result::Result;
//use web3::futures::Future;
//use web3::error::Error;
//
//fn InitConfig() -> Config{
//    Config::default()
//}
//
//pub fn getBlockNumber() -> Result<String,Error> {
//    let (_eloop, transport) = web3::transports::Http::new(&InitConfig().url.rpc)?;
//
//    let web3 = web3::Web3::new(transport);
//    let curNum = web3.eth().block_number().wait()?;
//    Ok(format!("{}",curNum))
//}
//
//#[test]
//fn getBlockNumber_test(){
//     let ss = getBlockNumber().unwrap();
//    println!("blockNumber =={}",ss);
//}