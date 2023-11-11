// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           30
// Async Callback:                       1
// Total number of exported functions:  32

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    router
    (
        pause
        resume
        createPair
        upgradePair
        issueLpToken
        setLocalRoles
        setLocalRolesOwner
        removePair
        setFeeOn
        setFeeOff
        setPairCreationEnabled
        getPairCreationEnabled
        getState
        getOwner
        getAllPairsManagedAddresses
        getAllPairTokens
        getAllPairContractMetadata
        getPair
        clearPairTemporaryOwnerStorage
        setTemporaryOwnerPeriod
        setPairTemplateAddress
        getPairTemplateAddress
        getTemporaryOwnerPeriod
        multiPairSwap
        configEnableByUserParameters
        addCommonTokensForUserPairs
        removeCommonTokensForUserPairs
        setSwapEnabledByUser
        getEnableSwapByUserConfig
        getCommonTokensForUserPairs
        callBack
    )
}
