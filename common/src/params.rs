

#[repr(C, packed(1))]
pub struct CtorParams {}

#[repr(C, packed(1))]
pub struct DtorParams {}

impl CtorParams {
    pub const METHOD: u32 = 0;
}

impl DtorParams {
    pub const METHOD: u32 = 1;
}
