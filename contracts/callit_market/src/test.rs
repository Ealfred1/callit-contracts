#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, BytesN, Env, String, Symbol,
};

#[test]
fn test_callit_market_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let creator = Address::generate(&env);
    let user_yes = Address::generate(&env);
    let user_no = Address::generate(&env);

    // Create settlement token (e.g. test USDC)
    let token_admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_contract_id.address());
    let token_admin_client = StellarAssetClient::new(&env, &token_contract_id.address());

    // Mint tokens to users (100 tokens = 100_000_000 stroops/base units)
    token_admin_client.mint(&user_yes, &100_000_000);
    token_admin_client.mint(&user_no, &100_000_000);

    // Register CallitMarket contract
    let contract_id = env.register(CallitMarketContract, ());
    let client = CallitMarketContractClient::new(&env, &contract_id);

    // Initialize
    client.initialize(&admin, &oracle, &token_contract_id.address());

    // Create market
    let market_id = Symbol::new(&env, "mkt_sol_1");
    let question = String::from_str(&env, "Will SOL reach $300 by Dec 2026?");
    let end_time = 1000;
    let fee_bps = 100; // 1%
    client.create_market(&creator, &market_id, &question, &end_time, &fee_bps);

    // Users buy positions
    // user_yes bets 60 tokens on YES
    client.buy_position(&user_yes, &market_id, &true, &60_000_000);
    // user_no bets 40 tokens on NO
    client.buy_position(&user_no, &market_id, &false, &40_000_000);

    let m = client.get_market(&market_id);
    assert_eq!(m.total_yes, 60_000_000);
    assert_eq!(m.total_no, 40_000_000);
    assert_eq!(m.status, MarketStatus::Active);

    // Fast-forward time past end_time
    env.ledger().set_timestamp(1500);

    // Oracle resolves market (YES wins) with AI Truth Consensus digest
    let digest: BytesN<32> = BytesN::from_array(&env, &[7u8; 32]);
    let evidence = String::from_str(&env, "https://callit.my/evidence/mkt_sol_1");
    client.resolve_market(&oracle, &market_id, &true, &digest, &evidence);

    let resolved_m = client.get_market(&market_id);
    assert_eq!(resolved_m.status, MarketStatus::Resolved);
    assert_eq!(resolved_m.winning_outcome, true);

    // Verify truth proof
    let truth = client.get_truth_proof(&market_id);
    assert_eq!(truth.outcome_yes, true);
    assert_eq!(truth.truth_digest, digest);

    // user_yes claims winnings:
    // Total pool = 100_000_000
    // Gross payout = (60m * 100m) / 60m = 100_000_000
    // Fee (1%) = 1_000_000
    // Net payout = 99_000_000
    let balance_before = token_client.balance(&user_yes);
    assert_eq!(balance_before, 40_000_000); // 100m - 60m spent

    let claimed_amount = client.claim(&user_yes, &market_id);
    assert_eq!(claimed_amount, 99_000_000);

    let balance_after = token_client.balance(&user_yes);
    assert_eq!(balance_after, 139_000_000); // 40m + 99m net payout

    // Verify position is marked claimed
    let pos = client.get_position(&market_id, &user_yes);
    assert_eq!(pos.claimed, true);
}
