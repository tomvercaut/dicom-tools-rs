/// Stores an unsigned and signed 16 bit value.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct UsSs {
    pub us: Option<u16>,
    pub ss: Option<i16>,
}
