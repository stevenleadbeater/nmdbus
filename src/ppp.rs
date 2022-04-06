// This code was autogenerated with `dbus-codegen-rust -c blocking --file target/nm-1.10.14/NetworkManager-1.10.14/introspection/org.freedesktop.NetworkManager.PPP.xml`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait PPP {
    fn need_secrets(&self) -> Result<(String, String), dbus::Error>;
    fn set_ip4_config(&self, config: arg::PropMap) -> Result<(), dbus::Error>;
    fn set_ip6_config(&self, config: arg::PropMap) -> Result<(), dbus::Error>;
    fn set_state(&self, state: u32) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> PPP for blocking::Proxy<'a, C> {

    fn need_secrets(&self) -> Result<(String, String), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.PPP", "NeedSecrets", ())
    }

    fn set_ip4_config(&self, config: arg::PropMap) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.PPP", "SetIp4Config", (config, ))
    }

    fn set_ip6_config(&self, config: arg::PropMap) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.PPP", "SetIp6Config", (config, ))
    }

    fn set_state(&self, state: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.PPP", "SetState", (state, ))
    }
}
