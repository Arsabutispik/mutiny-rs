pub trait Nameable {
    fn name(&self) -> Option<&str>;
}

pub trait ServerId {
    fn server_id(&self) -> Option<&str>;
}