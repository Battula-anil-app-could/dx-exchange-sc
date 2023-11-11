// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    pause_all
    (
        addPausableContracts
        removePausableContracts
        pauseSelected
        pauseAll
        resumeSelected
        resumeAll
        getPausableContracts
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}
