// This code was autogenerated with `dbus-codegen-rust -c blocking --file target/nm-1.10.14/NetworkManager-1.10.14/introspection/org.freedesktop.NetworkManager.Settings.xml`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Settings {
    fn list_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_connection_by_uuid(&self, uuid: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_connection(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_connection_unsaved(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), dbus::Error>;
    fn reload_connections(&self) -> Result<bool, dbus::Error>;
    fn save_hostname(&self, hostname: &str) -> Result<(), dbus::Error>;
    fn connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn hostname(&self) -> Result<String, dbus::Error>;
    fn can_modify(&self) -> Result<bool, dbus::Error>;
}

#[derive(Debug)]
pub struct SettingsPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for SettingsPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for SettingsPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for SettingsPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings";
}

#[derive(Debug)]
pub struct SettingsNewConnection {
    pub connection: dbus::Path<'static>,
}

impl arg::AppendAll for SettingsNewConnection {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.connection, i);
    }
}

impl arg::ReadAll for SettingsNewConnection {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsNewConnection {
            connection: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for SettingsNewConnection {
    const NAME: &'static str = "NewConnection";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings";
}

#[derive(Debug)]
pub struct SettingsConnectionRemoved {
    pub connection: dbus::Path<'static>,
}

impl arg::AppendAll for SettingsConnectionRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.connection, i);
    }
}

impl arg::ReadAll for SettingsConnectionRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsConnectionRemoved {
            connection: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for SettingsConnectionRemoved {
    const NAME: &'static str = "ConnectionRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Settings for blocking::Proxy<'a, C> {

    fn list_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "ListConnections", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_connection_by_uuid(&self, uuid: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "GetConnectionByUuid", (uuid, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_connection(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "AddConnection", (connection, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_connection_unsaved(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "AddConnectionUnsaved", (connection, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "LoadConnections", (filenames, ))
    }

    fn reload_connections(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "ReloadConnections", ())
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn save_hostname(&self, hostname: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "SaveHostname", (hostname, ))
    }

    fn connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "Connections")
    }

    fn hostname(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "Hostname")
    }

    fn can_modify(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "CanModify")
    }
}
