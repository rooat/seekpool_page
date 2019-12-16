fn main(){
    let coin = String::from("33000000000000000000000");
//    let len = &coin.len() -10;
//    let ssk = &coin[0..len];
    let bal = coin.parse::<u128>().unwrap();
    let balance = bal/10_u128.pow(18);
    println!("{}",balance as i32);
}