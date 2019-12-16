use serde::{Deserialize, Serialize};
use crate::{Address , FromStr,utils};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerDetail {
    pub owner : String,
    pub inviter : String,
    pub author : String,
    pub burnt_coin : String,
    pub reward_level : String,
    pub max_storage : String,
    pub set_block : String
}
impl MinerDetail {
    pub fn default() -> MinerDetail {
        MinerDetail {
            owner: "0x0000000000000000000000000000000000000000".into(),
            author: "0x0000000000000000000000000000000000000000".into(),
            inviter: "0x0000000000000000000000000000000000000000".into(),
            burnt_coin: "0".into(),
            reward_level: "0".into(),
            max_storage: "0".into(),
            set_block: "0".into()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miner {
    pub miner : String,
    pub author : String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  NewRegister {
    pub owner : String,
    pub author : String,
    pub pid : u64,
}

impl NewRegister {
    pub fn default() -> NewRegister {
        NewRegister{
            owner : "0x0000000000000000000000000000000000000000".into(),
            author : "0x0000000000000000000000000000000000000000".into(),
            pid : 0_u64
        }
    }
    pub fn new(miner : String, author: String) -> NewRegister {
        let pid = utils::calPID(&author);
        NewRegister{
            owner : miner,
            author: author,
            pid : pid
        }
    }
}

