mod clinfo;
mod display;
mod error;
mod storage;

use error::Result;

/// Finds all the OpenCL platforms and devices on a system.
///
/// It displays OpenCL platform information from `clGetPlatformInfo` and
/// OpenCL device information from `clGetDeviceInfo` for all the platforms and
/// devices.
fn main() -> Result<()> {
    let cl_state = clinfo::get_setup()?;
    display::display_opencl_state(&cl_state)?;

    Ok(())
}
