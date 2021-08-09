use ethers::prelude::*;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    types::{Address, H256},
    utils::Solc,
};
use std::convert::TryFrom;
// use tokio::prelude::*;
mod contracts;
mod get_troves;

#[tokio::main]
async fn main() {
    // connect provider
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/05357e1281e842ceb4ca24cb22003894")
            .expect("failed");

    let [trove_manager, sorted_troves] = contracts::get_contracts(&provider);
    println!("get troves call");
    get_troves::get_troves(sorted_troves, trove_manager).await;

    // if provide {
    //         println!("provider is connected");
    //     } else {
    //         println!("not connected");
    //     }
}
