
use core::mem::size_of_val;

use beam_bvm_interface::root::*;
use beam_bvm_util::{
    safe::{app::*, common::*}, 
    util::app::*
};
use common::params::*;

pub(crate) type ActionFunc = fn(cid: ContractID);
pub(crate) type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

// MANAGER ACTIONS

pub(crate)  fn on_action_create_contract(_unused: ContractID) {
    let args = CtorParams {};
    transaction::generate_kernel(
        0 as *const ContractID,
        CtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Create contract\0".as_ptr(),
        0,
    );
}

pub(crate)  fn on_action_destroy_contract(cid: ContractID) {
    let args = DtorParams {};
    transaction::generate_kernel(
        &cid,
        DtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Destroy contract\0".as_ptr(),
        0,
    );
}

pub(crate) fn on_action_view_contracts(_unused: ContractID) {
    enum_and_dump_contracts::enum_and_dump_contracts(&::common::contract_sid::SID);
}