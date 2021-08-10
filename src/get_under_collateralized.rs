use super::get_troves;
use ethers::{
    contract::Contract,
    providers::{Http, Provider},
};

async fn get_ETH_USD_price(price_feed: Contract<&Provider<Http>>) -> f64 {
    let (id, price, start, end, round): (u128, u128, u128, u128, u128) = price_feed
        .method::<_, _>("latestRoundData", ())
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");
    return price as f64 / 1e8;
}

async fn get_TCR(trove_manager: &Contract<&Provider<Http>>, price: f64) -> f64 {
    let tcr: u128 = trove_manager
        .method::<_, _>("getTCR", (price * 1e18) as u128)
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");
    return tcr as f64 / 1e18;
}

pub async fn run(
    trove_manager: &Contract<&Provider<Http>>,
    price_feed: Contract<&Provider<Http>>,
    troves: Vec<get_troves::Trove>,
) -> Vec<get_troves::Trove> {
    let mut to_liquidate: Vec<get_troves::Trove> = vec![];
    let eth_usd: f64 = get_ETH_USD_price(price_feed).await;
    let tcr = get_TCR(trove_manager, eth_usd).await;
    let mcr: f64;
    let stabilty_mode: bool;

    if tcr < 1.5 {
        stabilty_mode = true;
        mcr = 1.0;
    } else {
        stabilty_mode = false;
        mcr = 1.1;
    }

    for t in troves {
        let cr = t.collateral * eth_usd / t.debt;
        println!("{} %", cr);
        if cr < mcr && !stabilty_mode {
            to_liquidate.push(t);
        } else if cr <= mcr && stabilty_mode {
            to_liquidate.push(t);
        }
    }
    return to_liquidate;
}
