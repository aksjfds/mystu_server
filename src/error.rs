pub const TOO_MANY_REQUEST: u16 = 429;

pub enum ResErr {
    Any,
    Detail(u16),
}

impl From<()> for ResErr {
    #[allow(unused_variables)]
    fn from(value: ()) -> Self {
        ResErr::Any
    }
}
