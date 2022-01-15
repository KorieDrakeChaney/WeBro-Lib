pub trait Identity {
    fn identity() -> Self;
}

pub trait IdentityTransorm {
    fn identity_transform(&mut self);
}