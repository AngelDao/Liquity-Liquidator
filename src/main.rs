use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
// use tokio::prelude::*;
mod contracts;
mod get_troves;
mod get_under_collateralized;
mod liquidate;

#[tokio::main]
async fn main() {
    // connect provider
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/05357e1281e842ceb4ca24cb22003894")
            .expect("failed");
    // connect contracts
    let [trove_manager, sorted_troves, price_feed] = contracts::get_contracts(&provider);
    // get liquity troves
    let troves: Vec<get_troves::Trove> = get_troves::run(sorted_troves, &trove_manager).await;
    // get the troves under collateralization ratio 1.1
    let under_c_troves: Vec<get_troves::Trove> =
        get_under_collateralized::run(&trove_manager, price_feed, troves).await;
    // liquidate::run(under_c_troves).await;
}
