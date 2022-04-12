use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct DeviceType<'a>(pub(super) Cow<'a, str>);

impl<'a> DeviceType<'a> {
    pub fn into_owned(self) -> OwnedDeviceType {
        OwnedDeviceType(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedDeviceType(pub(crate) String);
