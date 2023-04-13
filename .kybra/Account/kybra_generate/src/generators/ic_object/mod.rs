use cdk_framework::nodes::ActExternalCanister;
use cdk_framework::ActCanisterMethod;
use quote::quote;

use crate::generators::ic_object::functions::accept_message::generate_accept_message;
use crate::generators::ic_object::functions::arg_data_raw::generate_arg_data_raw;
use crate::generators::ic_object::functions::arg_data_raw_size::generate_arg_data_raw_size;
use crate::generators::ic_object::functions::caller::generate_caller;
use crate::generators::ic_object::functions::candid_decode::generate_candid_decode;
use crate::generators::ic_object::functions::candid_encode::generate_candid_encode;
use crate::generators::ic_object::functions::canister_balance::generate_canister_balance;
use crate::generators::ic_object::functions::canister_balance128::generate_canister_balance128;
use crate::generators::ic_object::functions::clear_timer::generate_clear_timer;
use crate::generators::ic_object::functions::data_certificate::generate_data_certificate;
use crate::generators::ic_object::functions::id::generate_id;
use crate::generators::ic_object::functions::method_name::generate_method_name;
use crate::generators::ic_object::functions::msg_cycles_accept::generate_msg_cycles_accept;
use crate::generators::ic_object::functions::msg_cycles_accept128::generate_msg_cycles_accept128;
use crate::generators::ic_object::functions::msg_cycles_available::generate_msg_cycles_available;
use crate::generators::ic_object::functions::msg_cycles_available128::generate_msg_cycles_available128;
use crate::generators::ic_object::functions::msg_cycles_refunded::generate_msg_cycles_refunded;
use crate::generators::ic_object::functions::msg_cycles_refunded128::generate_msg_cycles_refunded128;
use crate::generators::ic_object::functions::notify_functions::generate_notify_functions;
use crate::generators::ic_object::functions::notify_raw::generate_notify_raw;
use crate::generators::ic_object::functions::notify_with_payment128_functions::generate_notify_with_payment128_functions;
use crate::generators::ic_object::functions::performance_counter::generate_performance_counter;
use crate::generators::ic_object::functions::print::generate_print;
use crate::generators::ic_object::functions::reject::generate_reject;
use crate::generators::ic_object::functions::reject_code::generate_reject_code;
use crate::generators::ic_object::functions::reject_message::generate_reject_message;
use crate::generators::ic_object::functions::reply::generate_reply;
use crate::generators::ic_object::functions::reply_raw::generate_reply_raw;
use crate::generators::ic_object::functions::set_certified_data::generate_set_certified_data;
use crate::generators::ic_object::functions::set_timer::generate_set_timer;
use crate::generators::ic_object::functions::set_timer_interval::generate_set_timer_interval;
use crate::generators::ic_object::functions::stable64_grow::generate_stable64_grow;
use crate::generators::ic_object::functions::stable64_read::generate_stable64_read;
use crate::generators::ic_object::functions::stable64_size::generate_stable64_size;
use crate::generators::ic_object::functions::stable64_write::generate_stable64_write;
use crate::generators::ic_object::functions::stable_b_tree_map::contains_key::generate_stable_b_tree_map_contains_key;
use crate::generators::ic_object::functions::stable_b_tree_map::get::generate_stable_b_tree_map_get;
use crate::generators::ic_object::functions::stable_b_tree_map::insert::generate_stable_b_tree_map_insert;
use crate::generators::ic_object::functions::stable_b_tree_map::is_empty::generate_stable_b_tree_map_is_empty;
use crate::generators::ic_object::functions::stable_b_tree_map::items::generate_stable_b_tree_map_items;
use crate::generators::ic_object::functions::stable_b_tree_map::keys::generate_stable_b_tree_map_keys;
use crate::generators::ic_object::functions::stable_b_tree_map::len::generate_stable_b_tree_map_len;
use crate::generators::ic_object::functions::stable_b_tree_map::remove::generate_stable_b_tree_map_remove;
use crate::generators::ic_object::functions::stable_b_tree_map::values::generate_stable_b_tree_map_values;
use crate::generators::ic_object::functions::stable_bytes::generate_stable_bytes;
use crate::generators::ic_object::functions::stable_grow::generate_stable_grow;
use crate::generators::ic_object::functions::stable_read::generate_stable_read;
use crate::generators::ic_object::functions::stable_size::generate_stable_size;
use crate::generators::ic_object::functions::stable_write::generate_stable_write;
use crate::generators::ic_object::functions::time::generate_time;
use crate::generators::ic_object::functions::trap::generate_trap;
use crate::py_ast::kybra_types::StableBTreeMapNode;

mod functions;

pub fn generate_ic_object(
    canister_methods: &Vec<ActCanisterMethod>,
    external_canisters: &Vec<ActExternalCanister>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let accept_message = generate_accept_message();
    let arg_data_raw = generate_arg_data_raw();
    let arg_data_raw_size = generate_arg_data_raw_size();
    let caller = generate_caller();
    let candid_decode = generate_candid_decode();
    let candid_encode = generate_candid_encode();
    let canister_balance = generate_canister_balance();
    let canister_balance128 = generate_canister_balance128();
    let clear_timer = generate_clear_timer();
    let data_certificate = generate_data_certificate();
    let id = generate_id();
    let method_name = generate_method_name();
    let msg_cycles_accept = generate_msg_cycles_accept();
    let msg_cycles_accept128 = generate_msg_cycles_accept128();
    let msg_cycles_available = generate_msg_cycles_available();
    let msg_cycles_available128 = generate_msg_cycles_available128();
    let msg_cycles_refunded = generate_msg_cycles_refunded();
    let msg_cycles_refunded128 = generate_msg_cycles_refunded128();
    let notify_functions = generate_notify_functions(external_canisters);
    let notify_raw = generate_notify_raw();
    let notify_with_payment128_functions =
        generate_notify_with_payment128_functions(external_canisters);
    let performance_counter = generate_performance_counter();
    let print = generate_print();
    let reject = generate_reject();
    let reject_code = generate_reject_code();
    let reject_message = generate_reject_message();
    let reply = generate_reply(canister_methods);
    let reply_raw = generate_reply_raw();
    let set_certified_data = generate_set_certified_data();
    let set_timer = generate_set_timer();
    let set_timer_interval = generate_set_timer_interval();
    let stable_bytes = generate_stable_bytes();
    let stable_grow = generate_stable_grow();
    let stable_read = generate_stable_read();
    let stable_size = generate_stable_size();
    let stable_write = generate_stable_write();
    let stable_b_tree_map_contains_key =
        generate_stable_b_tree_map_contains_key(stable_b_tree_map_nodes);
    let stable_b_tree_map_get = generate_stable_b_tree_map_get(stable_b_tree_map_nodes);
    let stable_b_tree_map_insert = generate_stable_b_tree_map_insert(stable_b_tree_map_nodes);
    let stable_b_tree_map_is_empty = generate_stable_b_tree_map_is_empty(stable_b_tree_map_nodes);
    let stable_b_tree_map_items = generate_stable_b_tree_map_items(stable_b_tree_map_nodes);
    let stable_b_tree_map_keys = generate_stable_b_tree_map_keys(stable_b_tree_map_nodes);
    let stable_b_tree_map_len = generate_stable_b_tree_map_len(stable_b_tree_map_nodes);
    let stable_b_tree_map_remove = generate_stable_b_tree_map_remove(stable_b_tree_map_nodes);
    let stable_b_tree_map_values = generate_stable_b_tree_map_values(stable_b_tree_map_nodes);
    let stable64_grow = generate_stable64_grow();
    let stable64_read = generate_stable64_read();
    let stable64_size = generate_stable64_size();
    let stable64_write = generate_stable64_write();
    let time = generate_time();
    let trap = generate_trap();

    quote! {
        #[pyclass(module = false, name = "ic")]
        #[derive(Debug, PyPayload)]
        struct Ic {}

        #[pyclass]
        impl Ic {
            #accept_message
            #arg_data_raw
            #arg_data_raw_size
            #caller
            #candid_decode
            #candid_encode
            #canister_balance
            #canister_balance128
            #clear_timer
            #data_certificate
            #id
            #method_name
            #msg_cycles_accept
            #msg_cycles_accept128
            #msg_cycles_available
            #msg_cycles_available128
            #msg_cycles_refunded
            #msg_cycles_refunded128
            #(#notify_functions)*
            #notify_raw
            #(#notify_with_payment128_functions)*
            #performance_counter
            #print
            #reject
            #reject_code
            #reject_message
            #reply
            #reply_raw
            #set_certified_data
            #set_timer
            #set_timer_interval
            #stable_bytes
            #stable_grow
            #stable_read
            #stable_size
            #stable_write
            #stable_b_tree_map_contains_key
            #stable_b_tree_map_get
            #stable_b_tree_map_insert
            #stable_b_tree_map_is_empty
            #stable_b_tree_map_items
            #stable_b_tree_map_keys
            #stable_b_tree_map_len
            #stable_b_tree_map_remove
            #stable_b_tree_map_values
            #stable64_grow
            #stable64_read
            #stable64_size
            #stable64_write
            #time
            #trap
        }

    }
}
