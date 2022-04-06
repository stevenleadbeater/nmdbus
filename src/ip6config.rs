// This code was autogenerated with `dbus-codegen-rust -c blocking --file target/nm-1.10.14/NetworkManager-1.10.14/introspection/org.freedesktop.NetworkManager.IP6Config.xml`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait IP6Config {
    fn addresses(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>)>, dbus::Error>;
    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn gateway(&self) -> Result<String, dbus::Error>;
    fn routes(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>, dbus::Error>;
    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn nameservers(&self) -> Result<Vec<Vec<u8>>, dbus::Error>;
    fn domains(&self) -> Result<Vec<String>, dbus::Error>;
    fn searches(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_options(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_priority(&self) -> Result<i32, dbus::Error>;
}

#[derive(Debug)]
pub struct IP6ConfigPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for IP6ConfigPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for IP6ConfigPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(IP6ConfigPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for IP6ConfigPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.IP6Config";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> IP6Config for blocking::Proxy<'a, C> {

    fn addresses(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Addresses")
    }

    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "AddressData")
    }

    fn gateway(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Gateway")
    }

    fn routes(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Routes")
    }

    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "RouteData")
    }

    fn nameservers(&self) -> Result<Vec<Vec<u8>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Nameservers")
    }

    fn domains(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Domains")
    }

    fn searches(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "Searches")
    }

    fn dns_options(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "DnsOptions")
    }

    fn dns_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP6Config", "DnsPriority")
    }
}
