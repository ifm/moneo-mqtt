use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Prefix<'a>(pub(super) Cow<'a, str>);

impl<'a> Prefix<'a> {
    pub fn into_owned(self) -> OwnedPrefix {
        OwnedPrefix(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedPrefix(pub(crate) String);
