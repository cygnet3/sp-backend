use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_create_log_stream(port_: i64) {
    wire_create_log_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_amount_stream(port_: i64) {
    wire_create_amount_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_scan_progress_stream(port_: i64) {
    wire_create_scan_progress_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_setup(
    port_: i64,
    files_dir: *mut wire_uint_8_list,
    scan_sk: *mut wire_uint_8_list,
    spend_pk: *mut wire_uint_8_list,
    birthday: u32,
    is_testnet: bool,
) {
    wire_setup_impl(port_, files_dir, scan_sk, spend_pk, birthday, is_testnet)
}

#[no_mangle]
pub extern "C" fn wire_reset_wallet(port_: i64) {
    wire_reset_wallet_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_start_nakamoto(port_: i64) {
    wire_start_nakamoto_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_restart_nakamoto(port_: i64) {
    wire_restart_nakamoto_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_peer_count(port_: i64) {
    wire_get_peer_count_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_scan_next_n_blocks(port_: i64, n: u32) {
    wire_scan_next_n_blocks_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_scan_to_tip(port_: i64) {
    wire_scan_to_tip_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_wallet_info(port_: i64) {
    wire_get_wallet_info_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_birthday(port_: i64) {
    wire_get_birthday_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_wallet_balance(port_: i64) {
    wire_get_wallet_balance_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_receiving_address(port_: i64) {
    wire_get_receiving_address_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_keys_from_seed(port_: i64, seedphrase: *mut wire_uint_8_list) {
    wire_get_keys_from_seed_impl(port_, seedphrase)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
