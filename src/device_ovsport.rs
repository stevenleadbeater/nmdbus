// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DeviceOvsPort {
}

#[derive(Debug)]
pub struct DeviceOvsPortPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for DeviceOvsPortPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for DeviceOvsPortPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceOvsPortPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceOvsPortPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.OvsPort";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DeviceOvsPort for blocking::Proxy<'a, C> {
}
