use std::borrow::Cow;

use crate::topic::*;

#[derive(Debug, getset::Getters)]
pub struct Topic<'a> {
    #[getset(get = "pub")]
    pub(crate) prefix: Prefix<'a>,

    #[getset(get = "pub")]
    pub(crate) client_id: ClientId,

    #[getset(get = "pub")]
    pub(crate) device_type: DeviceType<'a>,

    #[getset(get = "pub")]
    pub(crate) device_ip_addr: DeviceIpAddr<'a>,

    #[getset(get = "pub")]
    pub(crate) device_port: DevicePort,

    #[getset(get = "pub")]
    pub(crate) sensor_port: SensorPort<'a>,

    #[getset(get = "pub")]
    pub(crate) sensor_type: SensorType<'a>,

    #[getset(get = "pub")]
    pub(crate) data_source_name: DataSourceName<'a>,
}

impl<'a> Topic<'a> {
    pub fn parse(s: &'a str) -> Result<Self, TopicFromStrErr> {
        use std::str::FromStr;

        let tokens: Vec<&str> = s.split('/').collect();

        if tokens.len() != 8 {
            return Err(TopicFromStrErr::UnexpectedNumberOfTokens(8, tokens.len()));
        }

        let prefix = Prefix(Cow::Borrowed(tokens[0]));
        let client_id = ClientId(uuid::Uuid::parse_str(tokens[1])?);
        let device_type = DeviceType(Cow::Borrowed(tokens[2]));
        let device_ip_addr = DeviceIpAddr(Cow::Borrowed(tokens[3]));
        let device_port =
            DevicePort(u16::from_str(tokens[4]).map_err(TopicFromStrErr::FailedToParsePort)?);
        let sensor_port = SensorPort(Cow::Borrowed(tokens[5]));
        let sensor_type = SensorType(Cow::Borrowed(tokens[6]));
        let data_source_name = DataSourceName(Cow::Borrowed(tokens[7]));

        Ok({
            Topic {
                prefix,
                client_id,
                device_type,
                device_ip_addr,
                device_port,
                sensor_port,
                sensor_type,
                data_source_name,
            }
        })
    }

    pub fn into_owned(self) -> OwnedTopic {
        OwnedTopic::from(self)
    }
}

#[derive(Debug, getset::Getters)]
pub struct OwnedTopic {
    #[getset(get = "pub")]
    prefix: OwnedPrefix,

    #[getset(get = "pub")]
    client_id: ClientId,

    #[getset(get = "pub")]
    device_type: OwnedDeviceType,

    #[getset(get = "pub")]
    device_ip_addr: OwnedDeviceIpAddr,

    #[getset(get = "pub")]
    device_port: DevicePort,

    #[getset(get = "pub")]
    sensor_port: OwnedSensorPort,

    #[getset(get = "pub")]
    sensor_type: OwnedSensorType,

    #[getset(get = "pub")]
    data_source_name: OwnedDataSourceName,
}

impl From<Topic<'_>> for OwnedTopic {
    fn from(t: Topic<'_>) -> OwnedTopic {
        OwnedTopic {
            prefix: t.prefix.into_owned(),
            client_id: t.client_id,
            device_type: t.device_type.into_owned(),
            device_ip_addr: t.device_ip_addr.into_owned(),
            device_port: t.device_port,
            sensor_port: t.sensor_port.into_owned(),
            sensor_type: t.sensor_type.into_owned(),
            data_source_name: t.data_source_name.into_owned(),
        }
    }
}

impl OwnedTopic {
    pub fn parse(s: &str) -> Result<Self, TopicFromStrErr> {
        Topic::parse(s).map(Topic::into_owned)
    }
}
