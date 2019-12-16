use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  PageSize {
    pub fisrst_page : i32,
    pub cur_page : i32,
    pub total_count : i32,
}

impl PageSize {
    pub fn default() -> PageSize{
        PageSize{
            fisrst_page : 1,
            cur_page : 1,
            total_count : 0
        }
    }
    pub fn to_last(&mut self) {
        if(self.cur_page >1 ){
            self.cur_page -= 1;
        }else{
            self.cur_page = 1;
        }
    }
    pub fn to_next(&mut self) {
        self.cur_page += 1;
    }
}