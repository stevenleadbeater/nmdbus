// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Device {
    fn reapply(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, version_id: u64, flags: u32) -> Result<(), dbus::Error>;
    fn get_applied_connection(&self, flags: u32) -> Result<(::std::collections::HashMap<String, arg::PropMap>, u64), dbus::Error>;
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn delete(&self) -> Result<(), dbus::Error>;
    fn udi(&self) -> Result<String, dbus::Error>;
    fn interface(&self) -> Result<String, dbus::Error>;
    fn ip_interface(&self) -> Result<String, dbus::Error>;
    fn driver(&self) -> Result<String, dbus::Error>;
    fn driver_version(&self) -> Result<String, dbus::Error>;
    fn firmware_version(&self) -> Result<String, dbus::Error>;
    fn capabilities(&self) -> Result<u32, dbus::Error>;
    fn ip4_address(&self) -> Result<u32, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn state_reason(&self) -> Result<(u32, u32), dbus::Error>;
    fn active_connection(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn managed(&self) -> Result<bool, dbus::Error>;
    fn set_managed(&self, value: bool) -> Result<(), dbus::Error>;
    fn autoconnect(&self) -> Result<bool, dbus::Error>;
    fn set_autoconnect(&self, value: bool) -> Result<(), dbus::Error>;
    fn firmware_missing(&self) -> Result<bool, dbus::Error>;
    fn nm_plugin_missing(&self) -> Result<bool, dbus::Error>;
    fn device_type(&self) -> Result<u32, dbus::Error>;
    fn available_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn physical_port_id(&self) -> Result<String, dbus::Error>;
    fn mtu(&self) -> Result<u32, dbus::Error>;
    fn metered(&self) -> Result<u32, dbus::Error>;
    fn lldp_neighbors(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn real(&self) -> Result<bool, dbus::Error>;
    fn ip4_connectivity(&self) -> Result<u32, dbus::Error>;
    fn ip6_connectivity(&self) -> Result<u32, dbus::Error>;
}

#[derive(Debug)]
pub struct DeviceStateChanged {
    pub new_state: u32,
    pub old_state: u32,
    pub reason: u32,
}

impl arg::AppendAll for DeviceStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.new_state, i);
        arg::RefArg::append(&self.old_state, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for DeviceStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceStateChanged {
            new_state: i.read()?,
            old_state: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Device for blocking::Proxy<'a, C> {

    fn reapply(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, version_id: u64, flags: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Reapply", (connection, version_id, flags, ))
    }

    fn get_applied_connection(&self, flags: u32) -> Result<(::std::collections::HashMap<String, arg::PropMap>, u64), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "GetAppliedConnection", (flags, ))
    }

    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Disconnect", ())
    }

    fn delete(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Delete", ())
    }

    fn udi(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Udi")
    }

    fn interface(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Interface")
    }

    fn ip_interface(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "IpInterface")
    }

    fn driver(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Driver")
    }

    fn driver_version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "DriverVersion")
    }

    fn firmware_version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "FirmwareVersion")
    }

    fn capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Capabilities")
    }

    fn ip4_address(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Address")
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "State")
    }

    fn state_reason(&self) -> Result<(u32, u32), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "StateReason")
    }

    fn active_connection(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "ActiveConnection")
    }

    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Config")
    }

    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Dhcp4Config")
    }

    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip6Config")
    }

    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Dhcp6Config")
    }

    fn managed(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Managed")
    }

    fn autoconnect(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Autoconnect")
    }

    fn firmware_missing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "FirmwareMissing")
    }

    fn nm_plugin_missing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "NmPluginMissing")
    }

    fn device_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "DeviceType")
    }

    fn available_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "AvailableConnections")
    }

    fn physical_port_id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "PhysicalPortId")
    }

    fn mtu(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Mtu")
    }

    fn metered(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Metered")
    }

    fn lldp_neighbors(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "LldpNeighbors")
    }

    fn real(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Real")
    }

    fn ip4_connectivity(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Connectivity")
    }

    fn ip6_connectivity(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip6Connectivity")
    }

    fn set_managed(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager.Device", "Managed", value)
    }

    fn set_autoconnect(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager.Device", "Autoconnect", value)
    }
}
