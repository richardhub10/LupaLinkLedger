#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, String, Symbol};

// Define structure for land parcel entries
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LandTitle {
    pub title_id: u32,
    pub owner_did: Address,
    pub location_tag: String,
    pub tax_value_php: i128,
}

// Storage keys - FIXED: Using the modern symbol_short macro to avoid deprecation warnings
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct LupaLinkContract;

#[contractimpl]
impl LupaLinkContract {
    // Initializes the contract instance with an administrative LGU entity authority
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
    }

    // Registers a land asset title map on-chain under a validated identity
    pub fn register_title(
        env: Env,
        admin: Address,
        title_id: u32,
        owner_did: Address,
        location_tag: String,
        tax_value_php: i128,
    ) {
        // Authenticate administration entity
        admin.require_auth();
        let saved_admin: Address = env.storage().instance().get(&ADMIN_KEY).expect("Not initialized");
        if admin != saved_admin {
            panic!("Unauthorized admin user");
        }

        // FIXED: Using raw title_id (u32) directly as the storage key instead of forcing string conversions
        if env.storage().instance().has(&title_id) {
            panic!("Title identifier already registered");
        }

        let title = LandTitle {
            title_id,
            owner_did,
            location_tag,
            tax_value_php,
        };

        // FIXED: Storing with pure title_id key
        env.storage().instance().set(&title_id, &title);
    }

    // Atomic Transfer & Payment: Transfers cash tokens to seller and updates title registry state simultaneously
    pub fn transfer_and_pay(
        env: Env,
        stablecoin: Address,
        title_id: u32,
        buyer_did: Address,
    ) {
        // Require signature authorization from the buying entity
        buyer_did.require_auth();

        // FIXED: Directly querying the u32 title_id key
        if !env.storage().instance().has(&title_id) {
            panic!("Land title identifier not found");
        }

        // FIXED: Directly fetching using the u32 title_id key
        let mut title: LandTitle = env.storage().instance().get(&title_id).unwrap();
        let current_owner = title.owner_did.clone();
        let property_cost = title.tax_value_php;

        if buyer_did == current_owner {
            panic!("Buyer already owns this asset");
        }

        // Initialize Stellar standard token engine client and execute payment loop
        let token_client = token::Client::new(&env, &stablecoin);
        token_client.transfer(&buyer_did, &current_owner, &property_cost);

        // Mutate title state fields inside storage block
        title.owner_did = buyer_did;
        
        // FIXED: Updating with pure title_id key
        env.storage().instance().set(&title_id, &title);
    }

    // Public validation query endpoint for auditing individual parcels
    pub fn check_title(env: Env, title_id: u32) -> LandTitle {
        // FIXED: Querying storage with the pure title_id directly
        env.storage().instance()
            .get(&title_id)
            .expect("Deed mapping not found")
    }
}