use std::{ffi::CStr, fmt, mem::MaybeUninit, ops::Deref};

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPU")]
pub struct Gpu(InterfaceImpl);

unsafe impl Interface for Gpu {
    type Impl = ffi::IADLXGPU;
    type Vtable = ffi::IADLXGPUVtbl;
    const IID: &'static str = "IADLXGPU";
}

impl Gpu {
    #[doc(alias = "VendorId")]
    pub fn vendor_id(&self) -> Result<&str> {
        let mut name = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().VendorId.unwrap())(self.as_raw(), name.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, name)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "ASICFamilyType")]
    pub fn asic_family_type(&self) -> Result<ffi::ADLX_ASIC_FAMILY_TYPE> {
        let mut asic_family_type = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().ASICFamilyType.unwrap())(self.as_raw(), asic_family_type.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, asic_family_type)
    }
    #[doc(alias = "Type")]
    pub fn type_(&self) -> Result<ffi::ADLX_GPU_TYPE> {
        let mut type_ = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Type.unwrap())(self.as_raw(), type_.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, type_)
    }
    #[doc(alias = "IsExternal")]
    pub fn is_external(&self) -> Result<bool> {
        let mut is_external = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsExternal.unwrap())(self.as_raw(), is_external.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, is_external).map(|x| x != 0)
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u__name/#doxid-d-o-x-i-a-d-l-x-g-p-u-name>
    #[doc(alias = "Name")]
    pub fn name(&self) -> Result<&str> {
        let mut name = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Name.unwrap())(self.as_raw(), name.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, name)
            .map(|name| unsafe { CStr::from_ptr(name) }.to_str().unwrap())
    }
    #[doc(alias = "DriverPath")]
    pub fn driver_path(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().DriverPath.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "PNPString")]
    pub fn pnp_string(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().PNPString.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    // #[doc(alias = "HasDesktops")]
    // pub fn HasDesktops(&self) -> Result<()> {
    //     let mut x = MaybeUninit::uninit();
    //     let result = unsafe { (self.vtable().HasDesktops.unwrap())(self.as_raw(), x.as_mut_ptr()) };

    //     Error::from_result_with_assume_init_on_success(result, x)
    // }
    #[doc(alias = "TotalVRAM")]
    pub fn total_vram(&self) -> Result<u32> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().TotalVRAM.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
    }
    #[doc(alias = "VRAMType")]
    pub fn vram_type(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().VRAMType.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    // #[doc(alias = "BIOSInfo")]
    // pub fn BIOSInfo(&self) -> Result<()> {
    //     let mut x = MaybeUninit::uninit();
    //     let result = unsafe { (self.vtable().BIOSInfo.unwrap())(self.as_raw(), x.as_mut_ptr()) };

    //     Error::from_result_with_assume_init_on_success(result, x)
    // }
    #[doc(alias = "DeviceId")]
    pub fn device_id(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().DeviceId.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "RevisionId")]
    pub fn revision_id(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().RevisionId.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "SubSystemId")]
    pub fn sub_system_id(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().SubSystemId.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "SubSystemVendorId")]
    pub fn sub_system_vendor_id(&self) -> Result<&str> {
        let mut x = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().SubSystemVendorId.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
            .map(|x| unsafe { CStr::from_ptr(x) }.to_str().unwrap())
    }
    #[doc(alias = "UniqueId")]
    pub fn unique_id(&self) -> Result<i32> {
        let mut x = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().UniqueId.unwrap())(self.as_raw(), x.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, x)
    }

    /// Returns the PCI address (bus, device and function) extracted from [`Self::unique_id()`].  It
    /// is unknown under which circumstances this is valid.
    ///
    /// This is an alternative to calling [`ffi::IADLMappingVtbl::BdfFromADLXGPU`], which requires
    /// the ADL library to be present and loaded.
    pub fn pci_address_from_unique_id(&self) -> Option<PciAddress> {
        let unique_id = self.unique_id().ok()? as u32;

        let bus = (unique_id >> 8) & 0xFF;
        let device = (unique_id >> 3) & 0x1F;
        let function = unique_id & 0x07;

        Some(PciAddress {
            bus,
            device,
            function,
        })
    }
}

// TODO(Marijn): Test inheritance!
/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u1/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPU1")]
pub struct Gpu1(Gpu);

unsafe impl Interface for Gpu1 {
    type Impl = ffi::IADLXGPU1;
    type Vtable = ffi::IADLXGPU1Vtbl;
    const IID: &'static str = "IADLXGPU1";
}

// TODO: Autogenerate interface hierarchy chains?
impl Deref for Gpu1 {
    type Target = Gpu;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Gpu1 {
    #[doc(alias = "PCIBusType")]
    pub fn pci_bus_type(&self) -> Result<ffi::ADLX_PCI_BUS_TYPE> {
        let mut pci_bus_type = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().PCIBusType.unwrap())(self.as_raw(), pci_bus_type.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, pci_bus_type)
    }
    #[doc(alias = "PCIBusLaneWidth")]
    pub fn pci_bus_lane_width(&self) -> Result<u32> {
        let mut pci_bus_lane_width = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().PCIBusLaneWidth.unwrap())(self.as_raw(), pci_bus_lane_width.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, pci_bus_lane_width)
    }
    #[doc(alias = "MultiGPUMode")]
    pub fn multi_gpu_mode(&self) -> Result<ffi::ADLX_MGPU_MODE> {
        let mut multi_gpu_mode = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().MultiGPUMode.unwrap())(self.as_raw(), multi_gpu_mode.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, multi_gpu_mode)
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u1__product_name/#doxid-d-o-x-i-a-d-l-x-g-p-u1-product-name>
    #[doc(alias = "ProductName")]
    pub fn product_name(&self) -> Result<&str> {
        let mut product_name = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().ProductName.unwrap())(self.as_raw(), product_name.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, product_name)
            .map(|name| unsafe { CStr::from_ptr(name) }.to_str().unwrap())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(alias = "BDF")]
pub struct PciAddress {
    pub bus: u32,
    pub device: u32,
    pub function: u32,
}

impl fmt::Display for PciAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}.{}", self.bus, self.device, self.function,)
    }
}

impl fmt::Debug for PciAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PCI address {}", self)
    }
}
