// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    xbulk
    (
        init => init
        getOwners => owners
        addOwner => add_owner
        bulksend => bulksend
        bulksendSameAmount => bulksend_same_amount
        draw => draw
        nftDistribution => nft_distribution
        bulkBurn => bulk_burn
        dnsRegister => dns_register
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
