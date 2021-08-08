use ethers::prelude::*;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, H256},
    utils::Solc,
};
use std::convert::TryFrom;

fn main() {
    // connect provider
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/05357e1281e842ceb4ca24cb22003894");
    match provider {
        Ok(p) => println!("{:?}", p),
        Err(_) => println!("could not connect provider"),
    }

    // if provide {
    //         println!("provider is connected");
    //     } else {
    //         println!("not connected");
    //     }
}
