use opencl3::device::{device_type_text, CL_DEVICE_TYPE_ALL};
use opencl3::error_codes::ClError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! impl_getter_single(
    ($struct_name:ident, $field:ident: $field_type:ty) => {
        impl $struct_name {
            /// Getter of the
            #[doc = stringify!($field)]
            /// field
            #[allow(unused)]
            pub fn $field(&self) -> $field_type {
                self.$field.clone()
            }
        }
    }
);

macro_rules! impl_getters(
    ($struct_name:ident, $($field:ident: $field_type:ty,)+) => {
        $(
            impl_getter_single!($struct_name, $field: $field_type);
        )*
    }
);

/// Information about a [Platform](opencl3::platform::Platform)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PlatformInfo {
    name: String,
    version: String,
    vendor: String,
    profile: String,
    extensions: String,
    devices: Vec<DeviceInfo>,
}

impl_getters!(
    PlatformInfo,
    name: String,
    version: String,
    vendor: String,
    profile: String,
    extensions: String,
    devices: Vec<DeviceInfo>,
);

impl PlatformInfo {
    /// Create a new instance from the given opencl platform and devices
    ///
    /// See also [DeviceInfo::construct]
    pub fn construct(
        platform: &opencl3::platform::Platform,
        devices: &Vec<DeviceInfo>,
    ) -> Result<Self, ClError> {
        Ok(PlatformInfo {
            name: platform.name()?,
            version: platform.version()?,
            vendor: platform.vendor()?,
            profile: platform.profile()?,
            extensions: platform.extensions()?,
            devices: devices.clone(),
        })
    }
}

/// Contains information about a [Device](opencl3::device::Device)
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DeviceInfo {
    // VENDOR
    vendor: String,
    vendor_id: opencl3::device::cl_uint,
    vendor_id_text: String,
    // Device
    name: String,
    version: String,
    // TYPE
    r#type: opencl3::device::cl_device_type,
    type_text: String,
    // OTHER
    profile: String,
    extensions: String,
    opencl_c_version: String,
    svm_mem_capability: opencl3::device::cl_device_svm_capabilities,
}

impl_getters!(
    DeviceInfo,
    // VENDOR
    vendor: String,
    vendor_id: opencl3::device::cl_uint,
    vendor_id_text: String,
    // Device
    name: String,
    version: String,
    // TYPE
    r#type: opencl3::device::cl_device_type,
    type_text: String,
    // OTHER
    profile: String,
    extensions: String,
    opencl_c_version: String,
    svm_mem_capability: opencl3::device::cl_device_svm_capabilities,
);

impl DeviceInfo {
    /// Create new instance from given opencl device
    pub fn construct(device: &opencl3::device::Device) -> Result<Self, ClError> {
        Ok(Self {
            // VENDOR
            vendor: device.vendor()?,
            vendor_id: device.vendor_id()?,
            vendor_id_text: opencl3::device::vendor_id_text(device.vendor_id()?).into(),
            // DEVICE
            name: device.name()?,
            version: device.version()?,
            // TYPE
            r#type: device.dev_type()?,
            type_text: device_type_text(device.dev_type()?).into(),
            // OTHER
            profile: device.profile()?,
            extensions: device.extensions()?,
            opencl_c_version: device.opencl_c_version()?,
            svm_mem_capability: device.svm_mem_capability(),
        })
    }
}

/// The complete opencl state of the current machine
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ClState {
    platforms: Vec<PlatformInfo>,
}

impl ClState {
    /// Obtain all devices for any platform
    pub fn get_all_devices(&self) -> Vec<DeviceInfo> {
        self.platforms
            .iter()
            .map(|pltfm| pltfm.devices.clone())
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Obtains all platforms currently present
    pub fn get_platforms(&self) -> Vec<PlatformInfo> {
        self.platforms.clone()
    }
}

/// Constructs the complete state of the opencl setup of the current machine
pub fn get_setup() -> Result<ClState, ClError> {
    let mut platforms = vec![];

    for platform in opencl3::platform::get_platforms()? {
        let mut devices = vec![];
        for device_id in platform.get_devices(CL_DEVICE_TYPE_ALL)? {
            let device = opencl3::device::Device::new(device_id);
            let device_info = DeviceInfo::construct(&device)?;
            devices.push(device_info);
        }
        let platform_info = PlatformInfo::construct(&platform, &devices)?;
        platforms.push(platform_info);
    }

    Ok(ClState { platforms })
}
