/*
 * Example smart contract written in RUST
 *
 * Adopted from NEAR documentation by MLabs (www.mlabs.city)
 * https://near-docs.io/develop/Contract
 *
 */

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue
};

const DEFAULT_TITLE: &str = "This is the default title";
const DEFAULT_DESCRIPTION: &str = "This is the default descripton"; 
const DEFAULT_MEDIA: &str = "https://i.ibb.co/R6P5Ppz/MLabs-logo-1200x1200-1.png";
const DEFAULT_COPIES: u64 = 1;
//for future reference, this is a text-encoded svg, this is a handy tool for encoding
//svg, although you must prepend 'data:image/svg+xml,' ::  https://yoksel.github.io/url-encoder/ 
const DATA_IMAGE_SVG_MLABS_ICON: &str = "data:image/svg+xml,%3C%3Fxml version='1.0' standalone='no'%3F%3E%3C!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 20010904//EN' 'http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd'%3E%3Csvg version='1.0' xmlns='http://www.w3.org/2000/svg' width='300.000000pt' height='300.000000pt' viewBox='0 0 300.000000 300.000000' preserveAspectRatio='xMidYMid meet'%3E%3Cg transform='translate(0.000000,300.000000) scale(0.100000,-0.100000)'%0Afill='%23000000' stroke='none'%3E%3Cpath d='M1523 2111 c-73 -40 -140 -91 -193 -146 l-45 -47 60 38 c63 40 276%0A144 294 144 18 0 12 19 -8 30 -30 16 -52 12 -108 -19z'/%3E%3Cpath d='M1880 2115 c-162 -30 -410 -135 -590 -252 -87 -56 -310 -214 -310%0A-219 0 -1 66 29 146 67 236 114 462 182 652 196 l87 6 80 89 c119 133 117 128%0A59 127 -27 0 -83 -7 -124 -14z'/%3E%3Cpath d='M2470 1569 c-58 -23 -100 -85 -100 -146 0 -77 49 -118 183 -157 95%0A-28 123 -74 70 -116 -23 -19 -37 -21 -81 -17 -29 3 -73 17 -100 32 l-47 26%0A-22 -28 c-13 -15 -23 -30 -23 -33 0 -10 69 -49 114 -64 143 -49 266 14 266%0A136 0 72 -38 108 -154 143 -86 26 -126 51 -126 79 0 28 13 47 41 61 39 21 83%0A19 131 -6 l41 -21 23 29 c20 25 21 30 7 45 -35 39 -164 60 -223 37z'/%3E%3Cpath d='M270 1521 c0 -40 0 -40 42 -43 l42 -3 85 -197 c47 -109 89 -198 92%0A-198 3 0 43 87 90 194 l84 194 3 -204 2 -204 40 0 40 0 0 250 0 251 -67 -3%0A-67 -3 -60 -137 c-33 -76 -62 -138 -65 -138 -4 0 -33 63 -66 140 l-60 140 -67%0A0 -68 0 0 -39z'/%3E%3Cpath d='M880 1310 l0 -250 185 0 185 0 0 40 0 40 -145 0 -145 0 0 210 0 210%0A-40 0 -40 0 0 -250z'/%3E%3Cpath d='M1534 1483 c18 -43 47 -111 64 -150 l31 -73 -85 0 -84 0 15 36 c8 20%0A15 38 15 40 0 2 -20 4 -43 4 l-44 0 -51 -122 c-28 -68 -54 -131 -58 -140 -5%0A-15 0 -18 38 -18 l43 0 24 60 24 60 121 0 121 0 24 -60 24 -60 44 0 c23 0 43%0A2 43 4 0 2 -47 113 -103 247 l-104 244 -46 3 -46 3 33 -78z'/%3E%3Cpath d='M1880 1520 l0 -40 139 0 c114 0 143 -3 155 -16 27 -26 24 -71 -6 -96%0A-17 -15 -41 -18 -154 -18 l-134 0 0 -40 0 -40 138 0 c125 0 141 -2 157 -20 24%0A-27 22 -70 -5 -92 -19 -15 -42 -18 -156 -18 l-134 0 0 -40 0 -40 140 0 c155 0%0A178 6 224 60 33 40 36 119 7 164 -19 29 -19 30 0 54 26 34 26 116 -1 159 -33%0A55 -62 63 -225 63 l-145 0 0 -40z'/%3E%3Cpath d='M270 1205 l0 -145 40 0 40 0 0 145 0 145 -40 0 -40 0 0 -145z'/%3E%3C/g%3E%3C/svg%3E%0A";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    title: String,
    description: String,
    media: String,
    copies: u64,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

//do I want to import this to avoid orphan-like type problems
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token by MLabs".to_string(),
                symbol: "MLABS".to_string(),
                icon: Some(DATA_IMAGE_SVG_MLABS_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }


    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        //creates the initialized version of the Contract struct
        Self {
            title: DEFAULT_TITLE.to_string(),
            description: DEFAULT_DESCRIPTION.to_string(),
            media: DEFAULT_MEDIA.to_string(),
            copies: DEFAULT_COPIES,
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),            
        }
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub fn set_title(&mut self, new_title: String) {
        // Method caller must be contract owner
        require!(
            env::predecessor_account_id() == env::current_account_id(), 
            "Calling account can not access this method"
        );
        log!("Saving new title {}", new_title);
        self.title = new_title;
    }

    pub fn get_description(&self) -> String {
        return self.description.clone();
    }

    pub fn set_description(&mut self, new_description: String) {
        // Method caller must be contract owner
        require!(
            env::predecessor_account_id() == env::current_account_id(), 
            "Calling account can not access this method"
        );
        log!("Saving new description {}", new_description);
        self.description = new_description;
    }

    pub fn get_media(&self) -> String {
        return self.media.clone();
    }

    pub fn set_media(&mut self, new_media: String) {
        // Method caller must be contract owner
        require!(
            env::predecessor_account_id() == env::current_account_id(), 
            "Calling account can not access this method"
        );
        log!("Saving new description {}", new_media);
        self.media = new_media;
    }

    pub fn get_copies(&self) -> u64 {
        return self.copies.clone();
    }

    pub fn set_copies(&mut self, new_copies: u64) {
        // Method caller must be contract owner
        require!(
            env::predecessor_account_id() == env::current_account_id(), 
            "Calling account can not access this method"
        );
        log!("Saving new description {}", new_copies);
        self.copies = new_copies;
    }
    
    
    
    pub fn mk_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            title: Some(self.get_title()),
            description: Some(self.get_description()),
            media: Some(self.get_media()),
            media_hash: None,
            copies: Some(self.get_copies()),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }
    
   /*  type TokenMetadata = {
        title: string|null, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        description: string|null, // free-form description
        media: string|null, // URL to associated media, preferably to decentralized, content-addressed storage
        media_hash: string|null, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
        copies: number|null, // number of copies of this set of metadata in existence when token was minted.
        issued_at: number|null, // When token was issued or minted, Unix epoch in milliseconds
        expires_at: number|null, // When token expires, Unix epoch in milliseconds
        starts_at: number|null, // When token starts being valid, Unix epoch in milliseconds
        updated_at: number|null, // When token was last updated, Unix epoch in milliseconds
        extra: string|null, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        reference: string|null, // URL to an off-chain JSON file with more info.
        reference_hash: string|null // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
      } */

    #[payable]
    // only account owner can access => due to .mint() => "Caller 
    // must be the owner_id set during contract initialization." =>
    //https://docs.rs/near-contract-standards/latest/near_contract_standards/non_fungible_token/struct.NonFungibleToken.html
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
    ) -> Token {
        require!(
            env::predecessor_account_id() == env::current_account_id(), 
            "Calling account can not access this method"
        );
        let metadata = self.mk_metadata();
        self.tokens.internal_mint(token_id, receiver_id, Some(metadata))
    }
}

//the below section is necessary to call enumeration functions (e.g. see no of NFTs per
//held by an account, query nft data, and built-in nft functions)

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}


//TO-DO turn NFT into greeting ++ add time and date
//-add mint_nft_from_greeting pub fn
//add more updatable params
//clean up this code and push to Git
//clean up your notes


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
