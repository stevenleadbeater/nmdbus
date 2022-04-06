// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DeviceOlpcMesh {
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn companion(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn active_channel(&self) -> Result<u32, dbus::Error>;
}

#[derive(Debug)]
pub struct DeviceOlpcMeshPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for DeviceOlpcMeshPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for DeviceOlpcMeshPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceOlpcMeshPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceOlpcMeshPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.OlpcMesh";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DeviceOlpcMesh for blocking::Proxy<'a, C> {

    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.OlpcMesh", "HwAddress")
    }

    fn companion(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.OlpcMesh", "Companion")
    }

    fn active_channel(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.OlpcMesh", "ActiveChannel")
    }
}
