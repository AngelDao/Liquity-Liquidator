use ethers::{
    abi::{Abi, Tokenizable},
    contract::Contract,
    providers::{Http, Provider},
    types::Address,
    utils::format_ether,
};
use std::iter::Map;

struct Trove {
    collateral: u128,
    debt: u128,
    stake: u128,
    status: String,
    index: u128,
}

fn convert_to_trove(response: (u128, u128, u128, u8, u16)) {
    let (debt, collateral, stake, status, index) = response;
    let status_array = [""]
    let formatted_coll = collateral as f64 / 1e18;
    let formatted_stake = stake as f64 / 1e18;
    let formatted_debt = debt as f64 / 1e18;
    println!("collateral ETH {:?}", formatted_coll);
    println!("stake ETH {:?}", formatted_stake);
    println!("debt LUSD {:?}", formatted_debt);

}

pub async fn get_troves(
    sorted_troves: Contract<&Provider<Http>>,
    trove_manager: Contract<&Provider<Http>>,
) {
    // let troves = vec![];
    println!("entered");
    let first_trove: Address = sorted_troves
        .method::<_, Address>("getLast", ())
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");

    println!("{:?}", first_trove);
    let trove = trove_manager
        .method::<_, (u128, u128, u128, u8, u16)>("Troves", first_trove)
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");
    println!("{:?}", trove);
    convert_to_trove(trove);
}
