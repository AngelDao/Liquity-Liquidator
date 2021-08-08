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
// use std::{fmt::Debug, marker::PhantomData, sync::Arc};
mod troves;

// struct Hat<R> {}

fn main() {
    // connect provider
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/05357e1281e842ceb4ca24cb22003894");

    // hack should have type definition
    // let mut trove_manger: Contract<_>;
    // let mut sorted_troves: Contract<_>;
    match provider {
        // Ok(p) => sorted_troves = troves::get_sorted_troves(p),
        Ok(p) => troves::get_sorted_troves(p),
        Err(_) => println!("could not connect provider"),
    }

    // if provide {
    //         println!("provider is connected");
    //     } else {
    //         println!("not connected");
    //     }
}
