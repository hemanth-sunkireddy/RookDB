
pub struct TupleHeader {
    pub total_len: u32,
    pub attr_count: u32,
    pub nullmap_bytes: u32,
    pub flags: boolean
}

pub struct Tuple {
    pub header: TupleHeader,
    pub id: u32,
}