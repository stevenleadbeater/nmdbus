// This code was autogenerated with `dbus-codegen-rust -c blocking --file target/nm-1.10.14/NetworkManager-1.10.14/introspection/org.freedesktop.NetworkManager.Device.Tun.xml`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DeviceTun {
    fn owner(&self) -> Result<i64, dbus::Error>;
    fn group(&self) -> Result<i64, dbus::Error>;
    fn mode(&self) -> Result<String, dbus::Error>;
    fn no_pi(&self) -> Result<bool, dbus::Error>;
    fn vnet_hdr(&self) -> Result<bool, dbus::Error>;
    fn multi_queue(&self) -> Result<bool, dbus::Error>;
    fn hw_address(&self) -> Result<String, dbus::Error>;
}

#[derive(Debug)]
pub struct DeviceTunPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for DeviceTunPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for DeviceTunPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceTunPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceTunPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Tun";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DeviceTun for blocking::Proxy<'a, C> {

    fn owner(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "Owner")
    }

    fn group(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "Group")
    }

    fn mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "Mode")
    }

    fn no_pi(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "NoPi")
    }

    fn vnet_hdr(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "VnetHdr")
    }

    fn multi_queue(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "MultiQueue")
    }

    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Tun", "HwAddress")
    }
}
