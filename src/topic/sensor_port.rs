use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct SensorPort<'a>(pub(super) Cow<'a, str>);

impl<'a> SensorPort<'a> {
    pub fn into_owned(self) -> OwnedSensorPort {
        OwnedSensorPort(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedSensorPort(pub(crate) String);
