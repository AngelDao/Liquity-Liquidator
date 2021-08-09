#[path = "./ITroveManagerABI.rs"]
mod trove_manager;

#[path = "./ISortedTrovesABI.rs"]
mod sorted_troves;

pub fn sorted_troves() -> String {
    return sorted_troves::i_sorted_troves_abi();
}

pub fn trove_manager() -> String {
    return trove_manager::i_trove_manager_abi();
}
