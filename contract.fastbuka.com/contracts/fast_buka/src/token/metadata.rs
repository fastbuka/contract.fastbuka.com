use soroban_sdk::{Env};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let util = TokenUtils::new(e);
    util.metadata().set_metadata(&metadata);
}

// They are not used at the moment 👇

// pub fn read_decimal(e: &Env) -> u32 {
//     let util = TokenUtils::new(e);
//     util.metadata().get_metadata().decimal
// }

// pub fn read_name(e: &Env) -> String {
//     let util = TokenUtils::new(e);
//     util.metadata().get_metadata().name
// }

// pub fn read_symbol(e: &Env) -> String {
//     let util = TokenUtils::new(e);
//     util.metadata().get_metadata().symbol
// }


// pub fn read_metadata(e: &Env) -> TokenMetadata {
//     let util = TokenUtils::new(e);
//     util.metadata().get_metadata()
// }