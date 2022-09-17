use crate::{
    bvm_safe::*, 
    bvm_interface::root::{*, Env::*}
};
use core::mem::size_of_val;

#[repr(C)]
pub struct VarReaderEx<const FLEXIBLE: bool> {
    handle: u32,
}

impl<const FLEXIBLE: bool> VarReaderEx<FLEXIBLE> {
    fn enum_internal<K1, K2>(&mut self, key1: &K1, key1_size: u32, key2: &K2, key2_size: u32) {
        self.handle = vars_enum(key1 as *const K1, key1_size, key2 as *const K2, key2_size);
    }

    fn close_internal(&self) {
        if FLEXIBLE {
            if self.handle == 0 {
                return;
            }
        }
        vars_close(self.handle);
    }

    pub fn new<K1, K2>(key1: &K1, key2: &K2) -> VarReaderEx<FLEXIBLE> {
        let mut r: VarReaderEx<FLEXIBLE> = Default::default();
        r.enum_internal(
            key1,
            size_of_val(key1) as u32,
            key2,
            size_of_val(key2) as u32,
        );
        r
    }

    pub fn move_next<K, V>(
        &self,
        key: *mut K,
        key_size: &mut u32,
        val: *mut V,
        val_size: &mut u32,
        repeat: u8,
    ) -> bool {
        vars_move_next(self.handle, key, key_size, val, val_size, repeat) != 0
    }

    pub fn move_next_t<K, V>(&self, key: &mut K, value: &mut V) -> bool {
        loop {
            let mut key_size: u32 = size_of_val(key) as u32;
            let mut value_size: u32 = size_of_val(value) as u32;
            if !self.move_next(key, &mut key_size, value, &mut value_size, 0) {
                return false;
            }
            if size_of_val(key) as u32 == key_size && size_of_val(value) as u32 == value_size {
                break;
            }
        }
        true
    }

    pub fn read<K, V>(key: &K, value: &mut V) -> bool {
        let mut r: VarReader = Default::default();
        let mut key_size: u32 = size_of_val(key) as u32;
        r.enum_internal(key, key_size, key, key_size);

        let mut val_size: u32 = size_of_val(value) as u32;
        key_size = 0;
        r.move_next(0 as *mut usize, &mut key_size, value, &mut val_size, 0)
            && size_of_val(value) as u32 == val_size
    }

    pub fn r#enum<K, V>(&mut self, key: &K, value: &V) {
        self.close_internal();
        let key_size: u32 = size_of_val(key) as u32;
        let value_size: u32 = size_of_val(value) as u32;
        self.enum_internal(key, key_size, value, value_size);
    }
}

impl<const FLEXIBLE: bool> Drop for VarReaderEx<FLEXIBLE> {
    fn drop(&mut self) {
        self.close_internal()
    }
}

impl<const FLEXIBLE: bool> Default for VarReaderEx<FLEXIBLE> {
    fn default() -> Self {
        VarReaderEx {
            handle: Default::default(),
        }
    }
}

pub type VarReader = VarReaderEx<false>;

#[repr(C)]
pub struct LogReader {
    handle: u32,
    pub pos: HeightPos,
}

impl LogReader {
    pub fn new<K1, K2>(
        key1: &K1,
        key2: &K2,
        pos_min: *const HeightPos,
        pos_max: *const HeightPos,
    ) -> LogReader {
        LogReader {
            handle: logs_enum(
                key1,
                size_of_val(key1) as u32,
                key2,
                size_of_val(key2) as u32,
                pos_min,
                pos_max,
            ),
            pos: HeightPos {
                m_Height: 0,
                m_Pos: 0
            },
        }
    }

    pub fn move_next<K, V>(
        &mut self,
        key: *mut K,
        key_size: &mut u32,
        val: *mut V,
        val_size: &mut u32,
        repeat: u8,
    ) -> bool {
        logs_move_next(
            self.handle,
            key,
            key_size,
            val,
            val_size,
            &mut self.pos,
            repeat,
        ) != 0
    }

    pub fn move_next_t<K, V>(&mut self, key: &mut K, value: &mut V) -> bool {
        loop {
            let mut key_size: u32 = size_of_val(key) as u32;
            let mut value_size: u32 = size_of_val(value) as u32;
            if !self.move_next(key, &mut key_size, value, &mut value_size, 0) {
                return false;
            }
            if size_of_val(key) as u32 == key_size && size_of_val(value) as u32 == value_size {
                break;
            }
        }
        true
    }
}

impl Drop for LogReader {
    fn drop(&mut self) {
        logs_close(self.handle);
    }
}

#[repr(C, packed(1))]
struct SidCid {
    pub sid: ShaderID,
    pub cid: ContractID,
}

type KeySidCid = Key_T<SidCid>;

#[repr(C)]
struct ContractsWalker {
    pub key: KeySidCid,
    pub height: Height,
    pub reader: VarReaderEx<true>,
}

impl ContractsWalker {
    pub fn r#enum(&mut self, sid: &ShaderID) {
        let k0 = KeySidCid {
            m_Prefix: KeyPrefix {
                m_Tag: KeyTag_SidCid,
                m_Cid: Default::default()
            },
            m_KeyInContract: SidCid {
                cid: Default::default(),
                sid: *sid,
            },
            _phantom_0: Default::default()
        };
        let k1 = KeySidCid {
            m_Prefix: KeyPrefix { ..k0.m_Prefix },
            m_KeyInContract: SidCid {
                cid: [0xff; 32],
                ..k0.m_KeyInContract
            },
            _phantom_0: Default::default()
        };
        self.reader.r#enum(&k0, &k1);
    }

    pub fn move_next(&mut self) -> bool {
        if !self.reader.move_next_t(&mut self.key, &mut self.height) {
            return false;
        }
        self.height = u64::from_be(self.height);
        return true;
    }
}

pub fn enum_and_dump_contracts(sid: &ShaderID) {
    doc_add_array("contracts\0");

    let mut wlk = ContractsWalker {
        key: KeySidCid {
            m_Prefix: KeyPrefix { 
                m_Tag: KeyTag_Internal, 
                m_Cid: Default::default() 
            },
            m_KeyInContract: SidCid {
                cid: [0; 32],
                sid: [1; 32], // to avoid memset lib call
            },
            _phantom_0: Default::default()
        },
        height: 0,
        reader: VarReaderEx::<true> { handle: 0 },
    };
    wlk.r#enum(&sid);
    while wlk.move_next() {
        doc_add_group("\0");
        doc_add_blob(
            "cid\0",
            &wlk.key.m_KeyInContract.cid,
            size_of_val(&wlk.key.m_KeyInContract.cid) as u32,
        );
        doc_add_num64("height\0", wlk.height);
        doc_close_group();
    }
    doc_close_array();
}
