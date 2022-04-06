// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait IP4Config {
    fn addresses(&self) -> Result<Vec<Vec<u32>>, dbus::Error>;
    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn gateway(&self) -> Result<String, dbus::Error>;
    fn routes(&self) -> Result<Vec<Vec<u32>>, dbus::Error>;
    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn nameservers(&self) -> Result<Vec<u32>, dbus::Error>;
    fn nameserver_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn domains(&self) -> Result<Vec<String>, dbus::Error>;
    fn searches(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_options(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_priority(&self) -> Result<i32, dbus::Error>;
    fn wins_servers(&self) -> Result<Vec<u32>, dbus::Error>;
    fn wins_server_data(&self) -> Result<Vec<String>, dbus::Error>;
}

#[derive(Debug)]
pub struct IP4ConfigPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for IP4ConfigPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for IP4ConfigPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(IP4ConfigPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for IP4ConfigPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.IP4Config";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> IP4Config for blocking::Proxy<'a, C> {

    fn addresses(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Addresses")
    }

    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "AddressData")
    }

    fn gateway(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Gateway")
    }

    fn routes(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Routes")
    }

    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "RouteData")
    }

    fn nameservers(&self) -> Result<Vec<u32>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Nameservers")
    }

    fn nameserver_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "NameserverData")
    }

    fn domains(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Domains")
    }

    fn searches(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Searches")
    }

    fn dns_options(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "DnsOptions")
    }

    fn dns_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "DnsPriority")
    }

    fn wins_servers(&self) -> Result<Vec<u32>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "WinsServers")
    }

    fn wins_server_data(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "WinsServerData")
    }
}
