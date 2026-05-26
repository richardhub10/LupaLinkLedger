#![cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    // Test 1: Happy Path - Complete land issuance, payment routing, and title swap execution
    #[test]
    fn test_happy_path_mvp_execution() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, LupaLinkContract);
        let client = LupaLinkContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let seller = Address::generate(&env);
        let buyer = Address::generate(&env);
        
        // Mock a stablecoin token contract setup
        let token_admin = Address::generate(&env);
        let token_id = env.register_stellar_asset_contract(token_admin);
        let token_client = token::Client::new(&env, &token_id);

        // Initialize contract configurations
        client.initialize(&admin);

        let title_id = 1001u32;
        let location = String::from_str(&env, "Lot 12, San Fernando, Pampanga");
        let property_value: i128 = 500000;

        // Register initial asset state
        client.register_title(&admin, &title_id, &seller, &location, &property_value);

        // Execute payment execution and transfer swap parameters
        client.transfer_and_pay(&token_id, &title_id, &buyer);

        // Verify the asset tracking fields reflect the transaction changes
        let updated_title = client.check_title(&title_id);
        assert_eq!(updated_title.owner_did, buyer);
    }

    // Test 2: Edge Case Failure Scenario - Action must abort if land asset title identity mapping does not exist
    #[test]
    #[should_panic(expected = "Land title identifier not found")]
    fn test_transfer_non_existent_title_panics() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, LupaLinkContract);
        let client = LupaLinkContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let buyer = Address::generate(&env);
        let token_id = Address::generate(&env);

        client.initialize(&admin);

        // Invoking an unregistered reference id target should cause compilation panic paths
        client.transfer_and_pay(&token_id, &9999u32, &buyer);
    }

    // Test 3: State Verification Assertion - Storage updates correctly post registration
    #[test]
    fn test_state_verification_after_registration() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, LupaLinkContract);
        let client = LupaLinkContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let seller = Address::generate(&env);
        client.initialize(&admin);

        let title_id = 777u32;
        let location = String::from_str(&env, "BGC, Taguig City");
        let price: i128 = 2500000;

        client.register_title(&admin, &title_id, &seller, &location, &price);

        let title_record = client.check_title(&title_id);
        assert_eq!(title_record.title_id, 777u32);
        assert_eq!(title_record.tax_value_php, 2500000);
        assert_eq!(title_record.owner_did, seller);
    }

    // Test 4: Edge Case - Double initialization attempts should fail cleanly
    #[test]
    #[should_panic(expected = "Contract already initialized")]
    fn test_double_initialization_fails() {
        let env = Env::default();
        let contract_id = env.register_contract(None, LupaLinkContract);
        let client = LupaLinkContractClient::new(&env, &contract_id);

        let admin1 = Address::generate(&env);
        let admin2 = Address::generate(&env);

        client.initialize(&admin1);
        client.initialize(&admin2);
    }

    // Test 5: Edge Case - Re-registering duplicate titles must cause structural aborts
    #[test]
    #[should_panic(expected = "Title identifier already registered")]
    fn test_duplicate_title_registration_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, LupaLinkContract);
        let client = LupaLinkContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        client.initialize(&admin);

        let location = String::from_str(&env, "Davao City Plot");
        client.register_title(&admin, &101u32, &owner, &location, &100000);
        client.register_title(&admin, &101u32, &owner, &location, &100000);
    }
}