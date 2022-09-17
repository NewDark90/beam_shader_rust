/* automatically generated by rust-bindgen 0.58.1 */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub type c_void = ::core::ffi::c_void;
    pub type c_char = i8;
    pub type Height = u64;
    pub type Amount = u64;
    pub type AssetID = u32;
    pub type Timestamp = u64;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Secp_point_data {
        pub m_X: [u8; 32usize],
        pub m_Y: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Secp_point_dataEx {
        pub m_X: [u8; 32usize],
        pub m_Y: [u8; 32usize],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct HeightPos {
        pub m_Height: root::Height,
        pub m_Pos: u32,
    }
    pub type PubKey = root::Secp_point_data;
    pub type ContractID = [u8; 32usize];
    pub type ShaderID = [u8; 32usize];
    pub type HashValue = [u8; 32usize];
    pub type HashValue512 = u8;
    pub type Secp_scalar_data = [u8; 32usize];
    #[repr(C)]
    pub struct ILoadVarCallback__bindgen_vtable(root::c_void);
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ILoadVarCallback {
        pub vtable_: *const ILoadVarCallback__bindgen_vtable,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct FundsChange {
        pub m_Amount: root::Amount,
        pub m_Aid: root::AssetID,
        pub m_Consume: u8,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct SigRequest {
        pub m_pID: *const root::c_void,
        pub m_nID: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct HashObj {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Secp_scalar {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Secp_point {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct BlockHeader {
        pub _address: u8,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct BlockHeader_InfoBase {
        pub m_Timestamp: root::Timestamp,
        pub m_Kernels: root::HashValue,
        pub m_Definition: root::HashValue,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct BlockHeader_Info {
        pub _base: root::BlockHeader_InfoBase,
        pub m_Height: root::Height,
        pub m_Hash: root::HashValue,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct BlockHeader_Prefix {
        pub m_Height: root::Height,
        pub m_Prev: root::HashValue,
        pub m_ChainWork: root::HashValue,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BlockHeader_Element {
        pub _base: root::BlockHeader_InfoBase,
        pub m_PoW: root::BlockHeader_Element_PoW,
    }
    #[repr(C, packed)]
    #[derive(Copy, Clone)]
    pub struct BlockHeader_Element_PoW {
        pub m_pIndices: [u8; 104usize],
        pub m_pNonce: [u8; 8usize],
        pub m_Difficulty: u32,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BlockHeader_Full {
        pub _base: root::BlockHeader_Prefix,
        pub _base_1: root::BlockHeader_Element,
    }
    extern "C" {
        #[link_name = "\u{1}get_Hash"]
        pub fn BlockHeader_Full_get_Hash(
            this: *const root::BlockHeader_Full,
            out: *mut root::HashValue,
            pRules: *const root::HashValue,
        );
    }
    extern "C" {
        #[link_name = "\u{1}get_HashInternal"]
        pub fn BlockHeader_Full_get_HashInternal(
            this: *const root::BlockHeader_Full,
            out: *mut root::HashValue,
            bFull: bool,
            pRules: *const root::HashValue,
        );
    }
    extern "C" {
        #[link_name = "\u{1}TestDifficulty"]
        pub fn BlockHeader_Full_TestDifficulty(this: *const root::BlockHeader_Full) -> bool;
    }
    impl BlockHeader_Full {
        #[inline]
        pub unsafe fn get_Hash(&self, out: *mut root::HashValue, pRules: *const root::HashValue) {
            BlockHeader_Full_get_Hash(self, out, pRules)
        }
        #[inline]
        pub unsafe fn get_HashInternal(
            &self,
            out: *mut root::HashValue,
            bFull: bool,
            pRules: *const root::HashValue,
        ) {
            BlockHeader_Full_get_HashInternal(self, out, bFull, pRules)
        }
        #[inline]
        pub unsafe fn TestDifficulty(&self) -> bool {
            BlockHeader_Full_TestDifficulty(self)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct KeyTag {
        pub _address: u8,
    }
    pub const KeyTag_Internal: u8 = 0;
    pub const KeyTag_InternalStealth: u8 = 8;
    pub const KeyTag_LockedAmount: u8 = 1;
    pub const KeyTag_Refs: u8 = 2;
    pub const KeyTag_OwnedAsset: u8 = 3;
    pub const KeyTag_ShaderChange: u8 = 4;
    pub const KeyTag_SidCid: u8 = 16;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct KeySearchFlags {
        pub _address: u8,
    }
    pub const KeySearchFlags_Exact: u8 = 1;
    pub const KeySearchFlags_Bigger: u8 = 2;
    pub mod Merkle {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Node {
            pub m_OnRight: u8,
            pub m_Value: root::HashValue,
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Stream {
        pub _address: u8,
    }
    pub const Stream_Out: u32 = 0;
    pub const Stream_Error: u32 = 1;
    pub const s_NonceSlots: u32 = 256;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ApiVersion {
        pub _address: u8,
    }
    pub const ApiVersion_Current: u32 = 1;
    pub mod Env {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            pub fn Write(pData: *const root::c_void, nData: u32, iStream: u32);
        }
        extern "C" {
            pub fn Memcpy(
                pDst: *mut root::c_void,
                pSrc: *const root::c_void,
                size: u32,
            ) -> *mut root::c_void;
        }
        extern "C" {
            pub fn Memset(pDst: *mut root::c_void, val: u8, size: u32) -> *mut root::c_void;
        }
        extern "C" {
            pub fn Memcmp(p1: *const root::c_void, p2: *const root::c_void, size: u32) -> i32;
        }
        extern "C" {
            pub fn Memis0(p: *const root::c_void, size: u32) -> u8;
        }
        extern "C" {
            pub fn Strlen(sz: *const root::c_char) -> u32;
        }
        extern "C" {
            pub fn Strcmp(sz1: *const root::c_char, sz2: *const root::c_char) -> i32;
        }
        extern "C" {
            pub fn StackAlloc(size: u32) -> *mut root::c_void;
        }
        extern "C" {
            pub fn StackFree(size: u32);
        }
        extern "C" {
            pub fn Heap_Alloc(size: u32) -> *mut root::c_void;
        }
        extern "C" {
            pub fn Heap_Free(pPtr: *mut root::c_void);
        }
        extern "C" {
            pub fn Halt();
        }
        extern "C" {
            pub fn HashWrite(pHash: *mut root::HashObj, p: *const root::c_void, size: u32);
        }
        extern "C" {
            pub fn HashGetValue(pHash: *mut root::HashObj, pDst: *mut root::c_void, size: u32);
        }
        extern "C" {
            pub fn HashFree(pHash: *mut root::HashObj);
        }
        extern "C" {
            pub fn HashClone(pHash: *mut root::HashObj) -> *mut root::HashObj;
        }
        extern "C" {
            pub fn get_Height() -> root::Height;
        }
        extern "C" {
            pub fn get_HdrInfo(hdr: *mut root::BlockHeader_Info);
        }
        extern "C" {
            pub fn get_HdrFull(hdr: *mut root::BlockHeader_Full);
        }
        extern "C" {
            pub fn get_RulesCfg(h: root::Height, res: *mut root::HashValue) -> root::Height;
        }
        extern "C" {
            pub fn get_ForkHeight(iFork: u32) -> root::Height;
        }
        extern "C" {
            pub fn HashCreateSha256() -> *mut root::HashObj;
        }
        extern "C" {
            pub fn HashCreateBlake2b(
                pPersonal: *const root::c_void,
                nPersonal: u32,
                nResultSize: u32,
            ) -> *mut root::HashObj;
        }
        extern "C" {
            pub fn HashCreateKeccak(nBits: u32) -> *mut root::HashObj;
        }
        extern "C" {
            pub fn Secp_Scalar_alloc() -> *mut root::Secp_scalar;
        }
        extern "C" {
            pub fn Secp_Scalar_free(s: *mut root::Secp_scalar);
        }
        extern "C" {
            pub fn Secp_Scalar_import(
                s: *mut root::Secp_scalar,
                data: *const root::Secp_scalar_data,
            ) -> u8;
        }
        extern "C" {
            pub fn Secp_Scalar_export(
                s: *const root::Secp_scalar,
                data: *mut root::Secp_scalar_data,
            );
        }
        extern "C" {
            pub fn Secp_Scalar_neg(dst: *mut root::Secp_scalar, src: *const root::Secp_scalar);
        }
        extern "C" {
            pub fn Secp_Scalar_add(
                dst: *mut root::Secp_scalar,
                a: *const root::Secp_scalar,
                b: *const root::Secp_scalar,
            );
        }
        extern "C" {
            pub fn Secp_Scalar_mul(
                dst: *mut root::Secp_scalar,
                a: *const root::Secp_scalar,
                b: *const root::Secp_scalar,
            );
        }
        extern "C" {
            pub fn Secp_Scalar_inv(dst: *mut root::Secp_scalar, src: *const root::Secp_scalar);
        }
        extern "C" {
            pub fn Secp_Scalar_set(dst: *mut root::Secp_scalar, val: u64);
        }
        extern "C" {
            pub fn Secp_Point_alloc() -> *mut root::Secp_point;
        }
        extern "C" {
            pub fn Secp_Point_free(p: *mut root::Secp_point);
        }
        extern "C" {
            pub fn Secp_Point_Import(p: *mut root::Secp_point, pk: *const root::PubKey) -> u8;
        }
        extern "C" {
            pub fn Secp_Point_Export(p: *const root::Secp_point, pk: *mut root::PubKey);
        }
        extern "C" {
            pub fn Secp_Point_neg(dst: *mut root::Secp_point, src: *const root::Secp_point);
        }
        extern "C" {
            pub fn Secp_Point_add(
                dst: *mut root::Secp_point,
                a: *const root::Secp_point,
                b: *const root::Secp_point,
            );
        }
        extern "C" {
            pub fn Secp_Point_mul(
                dst: *mut root::Secp_point,
                p: *const root::Secp_point,
                s: *const root::Secp_scalar,
            );
        }
        extern "C" {
            pub fn Secp_Point_IsZero(p: *const root::Secp_point) -> u8;
        }
        extern "C" {
            pub fn Secp_Point_mul_G(dst: *mut root::Secp_point, s: *const root::Secp_scalar);
        }
        extern "C" {
            pub fn Secp_Point_mul_J(dst: *mut root::Secp_point, s: *const root::Secp_scalar);
        }
        extern "C" {
            pub fn Secp_Point_mul_H(
                dst: *mut root::Secp_point,
                s: *const root::Secp_scalar,
                aid: root::AssetID,
            );
        }
        extern "C" {
            pub fn Secp_Point_ExportEx(
                p: *const root::Secp_point,
                res: *mut root::Secp_point_dataEx,
            );
        }
        extern "C" {
            pub fn VerifyBeamHashIII(
                pInp: *const root::c_void,
                nInp: u32,
                pNonce: *const root::c_void,
                nNonce: u32,
                pSol: *const root::c_void,
                nSol: u32,
            ) -> u8;
        }
        extern "C" {
            pub fn LoadVar(
                pKey: *const root::c_void,
                nKey: u32,
                pVal: *mut root::c_void,
                nVal: u32,
                nType: u8,
            ) -> u32;
        }
        extern "C" {
            pub fn SaveVar(
                pKey: *const root::c_void,
                nKey: u32,
                pVal: *const root::c_void,
                nVal: u32,
                nType: u8,
            ) -> u32;
        }
        extern "C" {
            pub fn EmitLog(
                pKey: *const root::c_void,
                nKey: u32,
                pVal: *const root::c_void,
                nVal: u32,
                nType: u8,
            ) -> u32;
        }
        extern "C" {
            pub fn CallFar(
                cid: *const root::ContractID,
                iMethod: u32,
                pArgs: *mut root::c_void,
                nArgs: u32,
                bInheritContext: u8,
            );
        }
        extern "C" {
            pub fn get_CallDepth() -> u32;
        }
        extern "C" {
            pub fn get_CallerCid(iCaller: u32, cid: *mut root::ContractID);
        }
        extern "C" {
            pub fn UpdateShader(pVal: *const root::c_void, nVal: u32);
        }
        extern "C" {
            pub fn LoadVarEx(
                pKey: *mut root::c_void,
                nKey: *mut u32,
                nKeyBufSize: u32,
                pVal: *mut root::c_void,
                nVal: *mut u32,
                nType: u8,
                nSearchFlag: u8,
            );
        }
        extern "C" {
            pub fn AddSig(pubKey: *const root::PubKey);
        }
        extern "C" {
            pub fn FundsLock(aid: root::AssetID, amount: root::Amount);
        }
        extern "C" {
            pub fn FundsUnlock(aid: root::AssetID, amount: root::Amount);
        }
        extern "C" {
            pub fn RefAdd(cid: *const root::ContractID) -> u8;
        }
        extern "C" {
            pub fn RefRelease(cid: *const root::ContractID) -> u8;
        }
        extern "C" {
            pub fn AssetCreate(pMeta: *const root::c_void, nMeta: u32) -> root::AssetID;
        }
        extern "C" {
            pub fn AssetEmit(aid: root::AssetID, amount: root::Amount, bEmit: u8) -> u8;
        }
        extern "C" {
            pub fn AssetDestroy(aid: root::AssetID) -> u8;
        }
        extern "C" {
            pub fn SelectContext(bDependent: u8, nChargeNeeded: u32);
        }
        extern "C" {
            pub fn Vars_Enum(
                pKey0: *const root::c_void,
                nKey0: u32,
                pKey1: *const root::c_void,
                nKey1: u32,
            ) -> u32;
        }
        extern "C" {
            pub fn Vars_MoveNext(
                iSlot: u32,
                pKey: *mut root::c_void,
                nKey: *mut u32,
                pVal: *mut root::c_void,
                nVal: *mut u32,
                nRepeat: u8,
            ) -> u8;
        }
        extern "C" {
            pub fn Vars_Close(iSlot: u32);
        }
        extern "C" {
            pub fn VarGetProof(
                pKey: *const root::c_void,
                nKey: u32,
                ppVal: *mut *const root::c_void,
                pnVal: *mut u32,
                ppProof: *mut *const root::Merkle::Node,
            ) -> u32;
        }
        extern "C" {
            pub fn Logs_Enum(
                pKey0: *const root::c_void,
                nKey0: u32,
                pKey1: *const root::c_void,
                nKey1: u32,
                pPosMin: *const root::HeightPos,
                pPosMax: *const root::HeightPos,
            ) -> u32;
        }
        extern "C" {
            pub fn Logs_MoveNext(
                iSlot: u32,
                pKey: *mut root::c_void,
                nKey: *mut u32,
                pVal: *mut root::c_void,
                nVal: *mut u32,
                pos: *mut root::HeightPos,
                nRepeat: u8,
            ) -> u8;
        }
        extern "C" {
            pub fn Logs_Close(iSlot: u32);
        }
        extern "C" {
            pub fn LogGetProof(
                pos: *const root::HeightPos,
                ppProof: *mut *const root::Merkle::Node,
            ) -> u32;
        }
        extern "C" {
            pub fn DerivePk(pubKey: *mut root::PubKey, pID: *const root::c_void, nID: u32);
        }
        extern "C" {
            pub fn DocAddGroup(szID: *const root::c_char);
        }
        extern "C" {
            pub fn DocCloseGroup();
        }
        extern "C" {
            pub fn DocAddText(szID: *const root::c_char, val: *const root::c_char);
        }
        extern "C" {
            pub fn DocAddNum32(szID: *const root::c_char, val: u32);
        }
        extern "C" {
            pub fn DocAddNum64(szID: *const root::c_char, val: u64);
        }
        extern "C" {
            pub fn DocAddArray(szID: *const root::c_char);
        }
        extern "C" {
            pub fn DocCloseArray();
        }
        extern "C" {
            pub fn DocAddBlob(szID: *const root::c_char, pBlob: *const root::c_void, nBlob: u32);
        }
        extern "C" {
            pub fn DocGetText(
                szID: *const root::c_char,
                szRes: *mut root::c_char,
                nLen: u32,
            ) -> u32;
        }
        extern "C" {
            pub fn DocGetNum32(szID: *const root::c_char, pOut: *mut u32) -> u8;
        }
        extern "C" {
            pub fn DocGetNum64(szID: *const root::c_char, pOut: *mut u64) -> u8;
        }
        extern "C" {
            pub fn DocGetBlob(szID: *const root::c_char, pOut: *mut root::c_void, nLen: u32)
                -> u32;
        }
        extern "C" {
            pub fn GenerateKernel(
                pCid: *const root::ContractID,
                iMethod: u32,
                pArg: *const root::c_void,
                nArg: u32,
                pFunds: *const root::FundsChange,
                nFunds: u32,
                pSig: *const root::SigRequest,
                nSig: u32,
                szComment: *const root::c_char,
                nCharge: u32,
            );
        }
        extern "C" {
            pub fn GetApiVersion() -> u32;
        }
        extern "C" {
            pub fn SetApiVersion(nVer: u32);
        }
        extern "C" {
            pub fn GenerateRandom(pBuf: *mut root::c_void, nSize: u32);
        }
        extern "C" {
            pub fn get_SlotImage(res: *mut root::Secp_point, iSlot: u32);
        }
        extern "C" {
            pub fn SlotInit(pExtraSeed: *const root::c_void, nExtraSeed: u32, iSlot: u32);
        }
        extern "C" {
            pub fn get_Pk(res: *mut root::Secp_point, pID: *const root::c_void, nID: u32);
        }
        extern "C" {
            pub fn get_BlindSk(
                res: *mut root::Secp_scalar,
                pID: *const root::c_void,
                nID: u32,
                mul: *const root::Secp_scalar,
                iSlot: u32,
            );
        }
        extern "C" {
            pub fn GenerateKernelAdvanced(
                pCid: *const root::ContractID,
                iMethod: u32,
                pArg: *const root::c_void,
                nArg: u32,
                pFunds: *const root::FundsChange,
                nFunds: u32,
                pSig: *const root::PubKey,
                nSig: u32,
                szComment: *const root::c_char,
                nCharge: u32,
                hMin: root::Height,
                hMax: root::Height,
                ptFullBlind: *const root::PubKey,
                ptFullNonce: *const root::PubKey,
                skForeignSig: *const root::Secp_scalar_data,
                iSlotBlind: u32,
                iSlotNonce: u32,
                pChallenges: *mut root::Secp_scalar_data,
            );
        }
        extern "C" {
            pub fn get_SlotImageEx(
                res: *mut root::Secp_point,
                gen: *const root::Secp_point,
                iSlot: u32,
            );
        }
        extern "C" {
            pub fn get_PkEx(
                res: *mut root::Secp_point,
                gen: *const root::Secp_point,
                pID: *const root::c_void,
                nID: u32,
            );
        }
        extern "C" {
            pub fn Comm_Send(pkRemote: *const root::PubKey, pBuf: *const root::c_void, nSize: u32);
        }
        extern "C" {
            pub fn Comm_Read(
                pBuf: *mut root::c_void,
                nSize: u32,
                pCookie: *mut u32,
                bKeep: u8,
            ) -> u32;
        }
        extern "C" {
            pub fn Comm_WaitMsg(nTimeout_ms: u32);
        }
        extern "C" {
            pub fn Comm_Listen(pID: *const root::c_void, nID: u32, nCookie: u32);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct KeyID {
            pub _base: root::SigRequest,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct DocArray {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct DocGroup {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct KeyPrefix {
            pub m_Cid: root::ContractID,
            pub m_Tag: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Key_T<T> {
            pub m_Prefix: root::Env::KeyPrefix,
            pub m_KeyInContract: T,
            pub _phantom_0: ::core::marker::PhantomData<::core::cell::UnsafeCell<T>>,
        }
        pub type VarReader = u8;
        #[repr(C)]
        #[derive(Debug)]
        pub struct LogReader {
            pub m_Handle: u32,
            pub m_Pos: root::HeightPos,
        }
    }
    pub mod Utils {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct BlobOf<T> {
            pub m_Obj: *mut T,
            pub _phantom_0: ::core::marker::PhantomData<::core::cell::UnsafeCell<T>>,
        }
        pub mod details {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            extern "C" {
                #[link_name = "\u{1}nDigits"]
                pub static Number_Radix_nDigits: u32;
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct String {
            pub _address: u8,
        }
        pub type String_Decimal = u8;
        extern "C" {
            #[link_name = "\u{1}N"]
            pub static String_Radix_Digits_N: u32;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct String_Radix_DigitsMax {
            pub _address: u8,
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct HashProcessor {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct HashProcessor_Base {
        pub m_p: *mut root::HashObj,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct HashProcessor_Sha256 {
        pub _base: root::HashProcessor_Base,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct HashProcessor_Blake2b {
        pub _base: root::HashProcessor_Base,
    }
    pub mod Secp {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct UnaryMinus<T> {
            pub m_Src: *const T,
            pub _phantom_0: ::core::marker::PhantomData<::core::cell::UnsafeCell<T>>,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct Scalar {
            pub m_p: *mut root::Secp_scalar,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct Point {
            pub m_p: *mut root::Secp_point,
        }
        pub type Oracle = root::HashProcessor_Base;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Signature {
            pub m_NoncePub: root::Secp_point_data,
            pub m_kSig: root::Secp_scalar_data,
        }
    }
    pub const g_Beam2Groth: root::Amount = 100000000;
}
