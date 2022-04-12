use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct DataSourceName<'a>(pub(super) Cow<'a, str>);

impl<'a> DataSourceName<'a> {
    pub fn into_owned(self) -> OwnedDataSourceName {
        OwnedDataSourceName(self.0.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedDataSourceName(pub(crate) String);
