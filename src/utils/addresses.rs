use ethers::types::Address;

pub struct Contracts {
    pub i_sorted_trove: Address,
    pub i_trove_manager: Address,
}

pub fn contracts() -> Contracts {
    Contracts {
        i_sorted_trove: "0x8FdD3fbFEb32b28fb73555518f8b361bCeA741A6"
            .parse::<Address>()
            .expect("fail"),
        i_trove_manager: "0xA39739EF8b0231DbFA0DcdA07d7e29faAbCf4bb2"
            .parse::<Address>()
            .expect("fail"),
    }
}
