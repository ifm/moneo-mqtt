use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct SensorType<'a>(pub(super) Cow<'a, str>);

impl<'a> SensorType<'a> {
    pub fn into_owned(self) -> OwnedSensorType {
        OwnedSensorType(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedSensorType(pub(crate) String);
