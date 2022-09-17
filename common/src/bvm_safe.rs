
use crate::bvm_interface::root::*;
use ::core::ffi::c_void;

pub fn var_get_proof<K, V>(
    key: *const K,
    key_size: u32,
    val: *mut *const V,
    val_size: *mut u32,
    proof: *mut *const Merkle::Node,
) -> u32 {
    unsafe {
        Env::VarGetProof(
            key as *const c_void,
            key_size,
            val as *mut *const c_void,
            val_size,
            proof,
        )
    }
}

pub fn derive_pk<T>(pubkey: &mut PubKey, id: *const T, id_size: u32) {
    unsafe { Env::DerivePk(pubkey, id as *const c_void, id_size) }
}

pub fn funds_lock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsLock(aid, amount) }
}

pub fn funds_unlock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsUnlock(aid, amount) }
}

pub fn add_sig(pubkey: &PubKey) {
    unsafe { Env::AddSig(pubkey) }
}

pub fn halt_if(condition: bool) {
    unsafe {
        if condition {
            Env::Halt()
        }
    }
}

pub fn halt() {
    unsafe { Env::Halt() }
}

pub fn emit_log<K, V>(
    key: *const K,
    key_size: u32,
    value: *const V,
    value_size: u32,
    tag: u8,
) -> u32 {
    unsafe {
        Env::EmitLog(
            key as *const c_void,
            key_size,
            value as *const c_void,
            value_size,
            tag,
        )
    }
}

pub fn load_var<K, V>(
    key: *const K,
    key_size: u32,
    value: *mut V,
    value_size: u32,
    tag: u8,
) -> u32 {
    unsafe {
        Env::LoadVar(
            key as *const c_void,
            key_size,
            value as *mut c_void,
            value_size,
            tag,
        )
    }
}

pub fn del_var<K>(key: *const K, key_size: u32) -> u32 {
    unsafe {
        Env::SaveVar(
            key as *const c_void,
            key_size,
            0 as *const c_void,
            0,
            KeyTag_Internal,
        )
    }
}

pub fn save_var<K, V>(key: *const K, key_size: u32, val: *const V, val_size: u32, tag: u8) -> u32 {
    unsafe {
        Env::SaveVar(
            key as *const c_void,
            key_size,
            val as *const c_void,
            val_size,
            tag,
        )
    }
}

pub fn doc_add_text<V>(id: &str, val: *const V) {
    unsafe { Env::DocAddText(id.as_ptr() as *const c_char, val as *const c_char) }
}

pub fn doc_get_text<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetText(id.as_ptr() as *const c_char, val as *mut c_char, val_size) }
}

pub fn doc_add_num32(id: &str, val: u32) {
    unsafe { Env::DocAddNum32(id.as_ptr() as *const c_char, val) }
}

pub fn doc_get_num64(id: &str, out: *mut u64) -> u8 {
    unsafe { Env::DocGetNum64(id.as_ptr() as *const c_char, out) }
}

pub fn doc_get_num32(id: &str, out: *mut u32) -> u8 {
    unsafe { Env::DocGetNum32(id.as_ptr() as *const c_char, out) }
}

pub fn doc_add_num64(id: &str, val: u64) {
    unsafe { Env::DocAddNum64(id.as_ptr() as *const c_char, val) }
}

pub fn doc_add_blob<V>(id: &str, val: *const V, val_size: u32) {
    unsafe { Env::DocAddBlob(id.as_ptr() as *const c_char, val as *const c_void, val_size) }
}

pub fn doc_get_blob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetBlob(id.as_ptr() as *const c_char, val as *mut c_void, val_size) }
}

pub fn doc_add_group(id: &str) {
    unsafe { Env::DocAddGroup(id.as_ptr() as *const c_char) }
}

pub fn doc_close_group() {
    unsafe { Env::DocCloseGroup() }
}

pub fn doc_add_array(id: &str) {
    unsafe { Env::DocAddArray(id.as_ptr() as *const c_char) }
}

pub fn doc_close_array() {
    unsafe { Env::DocCloseArray() }
}

pub fn mem_is_0<T>(ptr: *const T, size: u32) -> u8 {
    unsafe { Env::Memis0(ptr as *const c_void, size) }
}

pub fn memset<V>(dst: *mut V, val: u8, size: u32) -> *mut c_void {
    unsafe { Env::Memset(dst as *mut c_void, val, size) }
}

pub fn memcpy<S, D>(dst: *mut D, src: *mut S, size: u32) -> *mut c_void {
    unsafe { Env::Memcpy(dst as *mut c_void, src as *mut c_void, size) }
}

pub fn memcmp<S, D>(p1: *const S, p2: *const D, size: u32) -> i32 {
    unsafe { Env::Memcmp(p1 as *const c_void, p2 as *const c_void, size) }
}

pub fn strlen<V>(p: *const V) -> u32 {
    unsafe { Env::Strlen(p as *const c_char) }
}

pub fn heap_alloc(size: u32) -> *mut c_void {
    unsafe { Env::Heap_Alloc(size) }
}

pub fn heap_free<V>(p: *mut V) {
    unsafe { Env::Heap_Free(p as *mut c_void) }
}

pub fn logs_close(slot: u32) {
    unsafe { Env::Logs_Close(slot) }
}

pub fn logs_enum<U, V>(
    key0: *const U,
    key0_size: u32,
    key1: *const V,
    key1_size: u32,
    pos_min: *const HeightPos,
    pos_max: *const HeightPos,
) -> u32 {
    unsafe {
        Env::Logs_Enum(
            key0 as *const c_void,
            key0_size,
            key1 as *const c_void,
            key1_size,
            pos_min,
            pos_max,
        )
    }
}

pub fn logs_move_next<K, V>(
    slot: u32,
    key: *mut K,
    key_size: *mut u32,
    val: *mut V,
    val_size: *mut u32,
    pos: *mut HeightPos,
    repeat: u8,
) -> u8 {
    unsafe {
        Env::Logs_MoveNext(
            slot,
            key as *mut c_void,
            key_size,
            val as *mut c_void,
            val_size,
            pos,
            repeat,
        )
    }
}

pub fn vars_close(slot: u32) {
    unsafe { Env::Vars_Close(slot) }
}

pub fn vars_enum<U, V>(key0: *const U, key0_size: u32, key1: *const V, key1_size: u32) -> u32 {
    unsafe {
        Env::Vars_Enum(
            key0 as *const c_void,
            key0_size,
            key1 as *const c_void,
            key1_size,
        )
    }
}

pub fn vars_move_next<K, V>(
    slot: u32,
    key: *mut K,
    key_size: *mut u32,
    val: *mut V,
    val_size: *mut u32,
    repeat: u8,
) -> u8 {
    unsafe {
        Env::Vars_MoveNext(
            slot,
            key as *mut c_void,
            key_size,
            val as *mut c_void,
            val_size,
            repeat,
        )
    }
}

pub fn generate_kernel<U, V>(
    cid: *const ContractID,
    method: u32,
    arg: *const U,
    arg_size: u32,
    funds: *const FundsChange,
    funds_size: u32,
    sigs: *const SigRequest,
    sigs_size: u32,
    comment: *const V,
    charge: u32,
) {
    unsafe {
        Env::GenerateKernel(
            cid,
            method,
            arg as *const c_void,
            arg_size,
            funds,
            funds_size,
            sigs,
            sigs_size,
            comment as *const c_char,
            charge,
        )
    }
}

pub fn secp_point_import(ptr: *mut Secp_point, pk: *const PubKey) -> u8 {
    unsafe { Env::Secp_Point_Import(ptr, pk) }
}

pub fn secp_point_export(ptr: *const Secp_point, pk: *mut PubKey) {
    unsafe { Env::Secp_Point_Export(ptr, pk) }
}

pub fn secp_point_add(dst: *mut Secp_point, a: *const Secp_point, b: *const Secp_point) {
    unsafe { Env::Secp_Point_add(dst, a, b) }
}

pub fn get_pk(ptr: *mut Secp_point, id_ptr: *const usize, id_size: u32) {
    unsafe { Env::get_Pk(ptr, id_ptr as *const c_void, id_size) }
}
