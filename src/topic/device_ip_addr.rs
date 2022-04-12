use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct DeviceIpAddr<'a>(pub(super) Cow<'a, str>);

impl<'a> DeviceIpAddr<'a> {
    pub fn into_owned(self) -> OwnedDeviceIpAddr {
        OwnedDeviceIpAddr(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedDeviceIpAddr(pub(crate) String);
