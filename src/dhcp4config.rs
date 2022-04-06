// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DHCP4Config {
    fn options(&self) -> Result<arg::PropMap, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DHCP4Config for blocking::Proxy<'a, C> {

    fn options(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.DHCP4Config", "Options")
    }
}
