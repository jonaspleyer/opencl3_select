// Copyright (c) 2021 Via Technology Ltd. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
