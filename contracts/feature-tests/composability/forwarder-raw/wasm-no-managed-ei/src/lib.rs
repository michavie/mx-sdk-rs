////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    forwarder_raw
    (
        callBack
        async_call_callback_data
        call_execute_on_dest_context
        call_execute_on_dest_context_by_caller
        call_execute_on_dest_context_readonly
        call_execute_on_dest_context_twice
        call_execute_on_same_context
        callback_data
        clear_callback_info
        deploy_contract
        deploy_from_source
        error_callback
        forward_async_call
        forward_async_call_half_payment
        forward_async_retrieve_multi_transfer_funds
        forward_direct_esdt_via_transf_exec
        forward_payment
        forward_register_promise
        forward_transf_exec
        forward_transf_exec_egld
        forward_transf_exec_esdt
        forwarder_async_send_and_retrieve_multi_transfer_funds
        forwarder_multi_transfer_via_promise
        success_callback
        upgrade
        upgrade_from_source
    )
}
