use libryzenadj::RyzenAdj;
use zbus_macros::dbus_interface;

use crate::performance::gpu::GraphicsCard;

pub struct AMDGPU {}

#[dbus_interface(name = "org.shadowblip.CPU")]
impl GraphicsCard for AMDGPU {
    #[dbus_interface(property)]
    fn name(&self) -> String {
        return "amd".to_string();
    }

    #[dbus_interface(property)]
    fn vendor(&self) -> String {
        todo!()
    }
}
