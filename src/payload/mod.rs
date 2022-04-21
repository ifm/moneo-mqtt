use serde_with::TimestampMilliSeconds;

#[derive(Debug, serde::Deserialize, getset::Getters)]
pub struct Payload {
    #[getset(get = "pub")]
    tid: ThingId,

    #[getset(get = "pub")]
    psid: DataSourceName,

    #[getset(get = "pub")]
    device_path: DevicePath,

    #[getset(get = "pub")]
    process_data_unit: ProcessDataUnit,

    #[getset(get = "pub")]
    values: Vec<Value>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ThingId(uuid::Uuid);

impl ThingId {
    pub fn into_uuid(self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DataSourceName(String);

impl DataSourceName {
    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DevicePath(String);

impl DevicePath {
    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProcessDataUnit(String);

impl ProcessDataUnit {
    pub fn into_string(self) -> String {
        self.0
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, getset::Getters)]
pub struct Value {
    #[getset(get = "pub")]
    value: serde_json::Value,

    #[getset(get = "pub")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl Value {
    pub fn into_inner(self) -> serde_json::Value {
        self.value
    }
}
