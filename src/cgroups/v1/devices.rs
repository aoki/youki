use std::{fs::create_dir_all, path::Path};

use anyhow::Result;
use nix::unistd::Pid;

use crate::cgroups::common;
use crate::{cgroups::v1::Controller, rootfs::default_devices};
use oci_spec::{LinuxDeviceCgroup, LinuxDeviceType, LinuxResources};

pub struct Devices {}

impl Controller for Devices {
    fn apply(linux_resources: &LinuxResources, cgroup_root: &Path, pid: Pid) -> Result<()> {
        log::debug!("Apply Devices cgroup config");
        create_dir_all(&cgroup_root)?;

        for d in &linux_resources.devices {
            Self::apply_device(d, cgroup_root)?;
        }

        for d in [
            default_devices().iter().map(|d| d.into()).collect(),
            Self::default_allow_devices(),
        ]
        .concat()
        {
            Self::apply_device(&d, &cgroup_root)?;
        }

        common::write_cgroup_file(cgroup_root.join("cgroup.procs"), &pid.to_string())?;
        Ok(())
    }
}

impl Devices {
    fn apply_device(device: &LinuxDeviceCgroup, cgroup_root: &Path) -> Result<()> {
        let path = if device.allow {
            cgroup_root.join("devices.allow")
        } else {
            cgroup_root.join("devices.deny")
        };

        common::write_cgroup_file(path, &device.to_string())?;
        Ok(())
    }

    fn default_allow_devices() -> Vec<LinuxDeviceCgroup> {
        vec![
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::C,
                major: None,
                minor: None,
                access: "m".to_string(),
            },
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::B,
                major: None,
                minor: None,
                access: "m".to_string(),
            },
            // /dev/console
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::C,
                major: Some(5),
                minor: Some(1),
                access: "rwm".to_string(),
            },
            // /dev/pts
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::C,
                major: Some(136),
                minor: None,
                access: "rwm".to_string(),
            },
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::C,
                major: Some(5),
                minor: Some(2),
                access: "rwm".to_string(),
            },
            // tun/tap
            LinuxDeviceCgroup {
                allow: true,
                typ: LinuxDeviceType::C,
                major: Some(10),
                minor: Some(200),
                access: "rwm".to_string(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_device_cgroup_to_string() {
        let ldc = LinuxDeviceCgroup {
            allow: true,
            typ: LinuxDeviceType::A,
            major: None,
            minor: None,
            access: "rwm".into(),
        };
        assert_eq!(ldc.to_string(), "a *:* rwm");
        let ldc = LinuxDeviceCgroup {
            allow: true,
            typ: LinuxDeviceType::A,
            major: Some(1),
            minor: Some(9),
            access: "rwm".into(),
        };
        assert_eq!(ldc.to_string(), "a 1:9 rwm");
    }
}
