// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback:                       1
// Total number of exported functions:  10

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    locked_token_wrapper
    (
        wrapLockedToken
        unwrapLockedToken
        issueWrappedToken
        setTransferRoleWrappedToken
        unsetTransferRoleWrappedToken
        getWrappedTokenId
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        callBack
    )
}
