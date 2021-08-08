const MAINNET: u8 = 1;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::Address,
};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// pub fn get_sorted_troves(provider: Provider<Http>) -> Contract<_> {
pub fn get_sorted_troves(provider: Provider<Http>) {
    // let paths = fs::read_dir("./src/").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    let path: &Path = Path::new("./src/abis/ISortedTroves.json");
    println!("{:?}", path.parent());
    let mut file: File = File::open(path).expect("cant open file");

    let mut abi = String::new();

    file.read_to_string(&mut abi)
        .expect("Oops!, cannot read file...");
    
        let contrac = 
}

// pub fn get_trove_manager(provider: Provider<Http>) -> Contract<_> {}
