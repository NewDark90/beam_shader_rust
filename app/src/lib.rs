#![no_std]
#![no_main]

use ::common::common::env;
use ::common::common::*;
use ::common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

type KeyAccount = env::Key<Key>;

fn dump_accounts(r: &mut env::VarReader) {
    env::doc_add_array("accounts\0");
    loop {
        let mut key = KeyAccount {
            prefix: env::KeyPrefix {
                cid: Default::default(),
                tag: KeyTag::INTERNAL,
            },
            key_in_contract: Key {
                account: SecpPointData {
                    x: Default::default(),
                    y: Default::default(),
                },
                aid: Default::default(),
            },
        };
        let mut amount: Amount = Default::default();

        if !r.move_next_t(&mut key, &mut amount) {
            break;
        }
        env::doc_add_group("\0");
        env::doc_add_blob(
            "Account\0",
            &key.key_in_contract.account,
            size_of_val(&key.key_in_contract.account) as u32,
        );
        env::doc_add_num32("AssetID\0", key.key_in_contract.aid);
        env::doc_add_num64("Amount\0", amount);
        env::doc_close_group();
    }
    env::doc_close_array();
}

fn dump_account(pubkey: &PubKey, cid: &ContractID) {
    let k0 = KeyAccount {
        prefix: env::KeyPrefix {
            cid: *cid,
            tag: KeyTag::INTERNAL,
        },
        key_in_contract: Key {
            account: *pubkey,
            aid: Default::default(),
        },
    };
    let k1 = KeyAccount {
        key_in_contract: Key {
            aid: AssetID::MAX,
            ..k0.key_in_contract
        },
        prefix: env::KeyPrefix { ..k0.prefix },
    };
    let mut r = env::VarReader::new(
        &k0 as *const KeyAccount as *const usize,
        size_of_val(&k0) as u32,
        &k1 as *const KeyAccount as *const usize,
        size_of_val(&k1) as u32,
    );
    dump_accounts(&mut r);
}
// MANAGER ACTIONS

fn on_action_create_contract(_unused: ContractID) {
    env::generate_kernel(
        0 as *const ContractID,
        0,
        0 as *const usize,
        0,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Create Vault contract\0".as_ptr(),
        0,
    );
}

fn on_action_destroy_contract(cid: ContractID) {
    env::generate_kernel(
        &cid,
        1,
        0 as *const usize,
        0,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Destroy Vault contract\0".as_ptr(),
        0,
    );
}

fn on_action_view_contracts(_unused: ContractID) {
    env::enum_and_dump_contracts(&::common::SID);
}

fn on_action_view_logs(cid: ContractID) {
    let k0 = KeyAccount {
        prefix: env::KeyPrefix {
            cid,
            tag: KeyTag::INTERNAL,
        },
        key_in_contract: Key {
            account: SecpPointData {
                x: Default::default(),
                y: Default::default(),
            },
            aid: Default::default(),
        },
    };
    let k1 = KeyAccount {
        prefix: env::KeyPrefix { ..k0.prefix },
        key_in_contract: Key {
            account: SecpPointData {
                x: [0xff; 32],
                y: 0xff,
            },
            aid: AssetID::MAX,
        },
    };

    let mut lr: env::LogReader = env::LogReader::new(
        &k0 as *const KeyAccount as *const usize,
        size_of_val(&k0) as u32,
        &k1 as *const KeyAccount as *const usize,
        size_of_val(&k1) as u32,
        0 as *const HeightPos,
        0 as *const HeightPos,
    );
    env::doc_add_array("logs\0");
    loop {
        let mut key = KeyAccount {
            prefix: env::KeyPrefix {
                cid,
                tag: KeyTag::INTERNAL,
            },
            key_in_contract: Key {
                account: SecpPointData {
                    x: Default::default(),
                    y: Default::default(),
                },
                aid: Default::default(),
            },
        };
        let mut val: Amount = Default::default();

        if !lr.move_next_t(&mut key, &mut val) {
            break;
        }

        env::doc_add_group("\0");
        env::doc_add_num64("Height\0", lr.pos.height);
        env::doc_add_num32("Pos\0", lr.pos.pos);
        env::doc_add_blob(
            "Account\0",
            &key.key_in_contract.account,
            size_of_val(&key.key_in_contract.account) as u32,
        );
        env::doc_add_num32("AssetID\0", key.key_in_contract.aid);
        env::doc_add_num64("Amount\0", val);
        env::doc_close_group();
    }
    env::doc_close_array();
}

fn on_action_view_accounts(cid: ContractID) {
    let k0 = env::KeyPrefix {
        cid,
        tag: KeyTag::INTERNAL,
    };
    let k1 = env::KeyPrefix {
        cid,
        tag: KeyTag::INTERNAL + 1,
    };
    let mut r = env::VarReader::new(
        &k0 as *const env::KeyPrefix as *const usize,
        size_of_val(&k0) as u32,
        &k1 as *const env::KeyPrefix as *const usize,
        size_of_val(&k1) as u32,
    );
    dump_accounts(&mut r);
}

fn on_action_view_account(cid: ContractID) {
    let mut pubkey = PubKey {
        x: Default::default(),
        y: Default::default(),
    };
    env::doc_get_blob("pubkey\0", &mut pubkey, size_of_val(&pubkey) as u32);
    dump_account(&pubkey, &cid);
}

// MY_ACCOUNT ACTIONS
fn on_action_view(_cid: ContractID) {}

fn on_action_get_key(_cid: ContractID) {}

fn on_action_get_proof(_cid: ContractID) {}

fn on_action_deposit(_cid: ContractID) {}

fn on_action_withdraw(_cid: ContractID) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {
    env::doc_add_group("\0");
    env::doc_add_group("roles\0");
    env::doc_add_group("manager\0");

    env::doc_add_group("create\0");
    env::doc_close_group(); // create

    env::doc_add_group("destroy\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // destroy

    env::doc_add_group("view\0");
    env::doc_close_group(); // view

    env::doc_add_group("view_account\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("pubkey\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // view_account

    env::doc_add_group("view_accounts\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view_accounts

    env::doc_add_group("view_logs\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view_logs

    env::doc_close_group(); // manager
    env::doc_add_group("my_account\0");

    env::doc_add_group("deposit\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("amount\0", "Amount\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_add_text("cosigner\0", "uint32_t\0".as_ptr());
    env::doc_add_text("foreign_key\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // deposit

    env::doc_add_group("get_key\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // get_key

    env::doc_add_group("get_proof\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_close_group(); // get_proof

    env::doc_add_group("view\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view

    env::doc_add_group("withdraw\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("amount\0", "Amount\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_add_text("cosigner\0", "uint32_t\0".as_ptr());
    env::doc_add_text("foreign_key\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // withdraw

    env::doc_close_group(); // my_account
    env::doc_close_group(); // roles
    env::doc_close_group(); // \0
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    const VALID_MY_ACCOUNT_ACTIONS: [(&str, ActionFunc); 5] = [
        ("view\0", on_action_view),
        ("get_key\0", on_action_get_key),
        ("get_proof\0", on_action_get_proof),
        ("deposit\0", on_action_deposit),
        ("withdraw\0", on_action_withdraw),
    ];

    const VALID_MANAGER_ACTIONS: [(&str, ActionFunc); 6] = [
        ("create\0", on_action_create_contract),
        ("destroy\0", on_action_destroy_contract),
        ("view\0", on_action_view_contracts),
        ("view_logs\0", on_action_view_logs),
        ("view_accounts\0", on_action_view_accounts),
        ("view_account\0", on_action_view_account),
    ];

    const VALID_ROLES: [(&str, ActionsMap); 2] = [
        ("my_account\0", &VALID_MY_ACCOUNT_ACTIONS),
        ("manager\0", &VALID_MANAGER_ACTIONS),
    ];

    let mut role: [u8; 32] = Default::default();
    env::doc_get_text("role\0", &mut role, size_of_val(&role) as u32);

    let mut action_map: ActionsMap = &INVALID_ROLE_ACTIONS;
    for i in 0..VALID_ROLES.len() {
        if env::memcmp(
            &role,
            VALID_ROLES[i].0.as_ptr(),
            VALID_ROLES[i].0.len() as u32,
        ) == 0
        {
            action_map = VALID_ROLES[i].1;
            break;
        }
    }

    if action_map == &INVALID_ROLE_ACTIONS {
        env::doc_add_text("error\0", "Invalid role\0".as_ptr());
        return;
    }

    let mut action: [u8; 32] = Default::default();
    env::doc_get_text("action\0", &mut action, size_of_val(&action) as u32);

    for i in 0..action_map.len() {
        if env::memcmp(
            &action,
            action_map[i].0.as_ptr(),
            action_map[i].0.len() as u32,
        ) == 0
        {
            let mut cid: ContractID = Default::default();
            env::doc_get_blob("cid\0", &mut cid, size_of_val(&cid) as u32);
            action_map[i].1(cid);
            return;
        }
    }

    env::doc_add_text("error\0", "Invalid action\0".as_ptr());
}
