// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           66
// Async Callback:                       1
// Total number of exported functions:  68

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    farm_with_locked_rewards
    (
        init => init
        enterFarm => enter_farm_endpoint
        claimRewards => claim_rewards_endpoint
        exitFarm => exit_farm_endpoint
        mergeFarmTokens => merge_farm_tokens_endpoint
        claimBoostedRewards => claim_boosted_rewards
        startProduceRewards => start_produce_rewards_endpoint
        endProduceRewards => end_produce_rewards_endpoint
        setPerBlockRewardAmount => set_per_block_rewards_endpoint
        calculateRewardsForGivenPosition => calculate_rewards_for_given_position
        getRewardPerShare => reward_per_share
        getRewardReserve => reward_reserve
        allowExternalClaimBoostedRewards => allow_external_claim_boosted_rewards
        getFarmingTokenId => farming_token_id
        getRewardTokenId => reward_token_id
        getPerBlockRewardAmount => per_block_reward_amount
        getLastRewardBlockNonce => last_reward_block_nonce
        getDivisionSafetyConstant => division_safety_constant
        getUserTotalFarmPosition => user_total_farm_position
        getFarmPositionMigrationNonce => farm_position_migration_nonce
        setLockingScAddress => set_locking_sc_address
        setLockEpochs => set_lock_epochs
        getLockingScAddress => locking_sc_address
        getLockEpochs => lock_epochs
        registerFarmToken => register_farm_token
        getFarmTokenId => farm_token
        getFarmTokenSupply => farm_token_supply
        addToPauseWhitelist => add_to_pause_whitelist
        removeFromPauseWhitelist => remove_from_pause_whitelist
        pause => pause
        resume => resume
        getState => state
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
        addSCAddressToWhitelist => add_sc_address_to_whitelist
        removeSCAddressFromWhitelist => remove_sc_address_from_whitelist
        isSCAddressWhitelisted => is_sc_address_whitelisted
        set_penalty_percent => set_penalty_percent
        set_minimum_farming_epochs => set_minimum_farming_epochs
        set_burn_gas_limit => set_burn_gas_limit
        getPenaltyPercent => penalty_percent
        getMinimumFarmingEpoch => minimum_farming_epochs
        getBurnGasLimit => burn_gas_limit
        getPairContractManagedAddress => pair_contract_address
        setBoostedYieldsRewardsPercentage => set_boosted_yields_rewards_percentage
        collectUndistributedBoostedRewards => collect_undistributed_boosted_rewards
        getBoostedYieldsRewardsPercentage => boosted_yields_rewards_percentage
        getAccumulatedRewardsForWeek => accumulated_rewards_for_week
        getFarmSupplyForWeek => farm_supply_for_week
        getRemainingBoostedRewardsToDistribute => remaining_boosted_rewards_to_distribute
        getUndistributedBoostedRewards => undistributed_boosted_rewards
        setBoostedYieldsFactors => set_boosted_yields_factors
        getBoostedYieldsFactors => get_boosted_yields_factors
        getCurrentWeek => get_current_week
        getFirstWeekStartEpoch => first_week_start_epoch
        getLastActiveWeekForUser => get_last_active_week_for_user_view
        getUserEnergyForWeek => get_user_energy_for_week_view
        getLastGlobalUpdateWeek => last_global_update_week
        getTotalRewardsForWeek => total_rewards_for_week
        getTotalEnergyForWeek => total_energy_for_week
        getTotalLockedTokensForWeek => total_locked_tokens_for_week
        updateEnergyForUser => update_energy_for_user
        getCurrentClaimProgress => current_claim_progress
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
    )
}

dharitri_sc_wasm_adapter::async_callback! { farm_with_locked_rewards }
