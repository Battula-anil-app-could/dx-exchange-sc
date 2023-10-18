// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback:                       1
// Total number of exported functions:  17

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    farm_staking_proxy
    (
        init => init
        registerDualYieldToken => register_dual_yield_token
        getDualYieldTokenId => dual_yield_token
        getLpFarmAddress => lp_farm_address
        getStakingFarmAddress => staking_farm_address
        getPairAddress => pair_address
        getStakingTokenId => staking_token_id
        getFarmTokenId => staking_farm_token_id
        getLpTokenId => lp_token_id
        getLpFarmTokenId => lp_farm_token_id
        addSCAddressToWhitelist => add_sc_address_to_whitelist
        removeSCAddressFromWhitelist => remove_sc_address_from_whitelist
        isSCAddressWhitelisted => is_sc_address_whitelisted
        stakeFarmTokens => stake_farm_tokens
        claimDualYield => claim_dual_yield_endpoint
        unstakeFarmTokens => unstake_farm_tokens
    )
}

dharitri_sc_wasm_adapter::async_callback! { farm_staking_proxy }
