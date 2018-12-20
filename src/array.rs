#[repr(C)]
#[derive(Debug)]
pub struct Array<T> {
    pub pointer: *const T,
    pub length: usize,
    pub capacity: usize,
}
