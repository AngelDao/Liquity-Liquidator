use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::Address,
};
#[path = "./utils/abis/abis.rs"]
mod abis;
#[path = "./utils/addresses.rs"]
mod addresses;

fn get_sorted_troves(provider: &Provider<Http>) -> Contract<&Provider<Http>> {
    let abi_original: String = abis::sorted_troves();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::contracts()).i_sorted_trove;
    let contract = Contract::new(address, abi, provider);
    return contract;
}

fn get_trove_manager(provider: &Provider<Http>) -> Contract<&Provider<Http>> {
    let abi_original: String = abis::trove_manager();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::contracts()).i_trove_manager;
    let contract = Contract::new(address, abi, provider);
    return contract;
}

pub fn get_contracts(provider: &Provider<Http>) -> [Contract<&Provider<Http>>; 2] {
    let static_provider = &provider;
    let trove_manager = get_trove_manager(static_provider);
    let sorted_troves = get_sorted_troves(static_provider);
    return [trove_manager, sorted_troves];
}
