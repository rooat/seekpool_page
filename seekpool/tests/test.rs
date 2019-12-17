extern crate web3;
use web3::futures::Future;

#[test]
fn test_web3() {
    let (_eloop, transport) = web3::transports::Http::new("http://13.251.6.203:8545").unwrap();

    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().block_number().wait().unwrap();

    println!("Accounts: {:?}", accounts);
}