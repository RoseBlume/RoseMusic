pub trait IDirectInput2A_Impl: Sized + IDirectInputA_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput2A {}
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput2A_Vtbl
    where
        Identity: IDirectInput2A_Impl,
    {
        unsafe extern "system" fn FindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput2A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self { base__: IDirectInputA_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2A as windows_core::Interface>::IID || iid == &<IDirectInputA as windows_core::Interface>::IID
    }
}
pub trait IDirectInput2W_Impl: Sized + IDirectInputW_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput2W {}
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput2W_Vtbl
    where
        Identity: IDirectInput2W_Impl,
    {
        unsafe extern "system" fn FindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput2W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self { base__: IDirectInputW_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2W as windows_core::Interface>::IID || iid == &<IDirectInputW as windows_core::Interface>::IID
    }
}
pub trait IDirectInput7A_Impl: Sized + IDirectInput2A_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput7A {}
impl IDirectInput7A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput7A_Vtbl
    where
        Identity: IDirectInput7A_Impl,
    {
        unsafe extern "system" fn CreateDeviceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput7A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput7A_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        Self { base__: IDirectInput2A_Vtbl::new::<Identity, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput7A as windows_core::Interface>::IID || iid == &<IDirectInputA as windows_core::Interface>::IID || iid == &<IDirectInput2A as windows_core::Interface>::IID
    }
}
pub trait IDirectInput7W_Impl: Sized + IDirectInput2W_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput7W {}
impl IDirectInput7W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput7W_Vtbl
    where
        Identity: IDirectInput7W_Impl,
    {
        unsafe extern "system" fn CreateDeviceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput7W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput7W_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        Self { base__: IDirectInput2W_Vtbl::new::<Identity, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput7W as windows_core::Interface>::IID || iid == &<IDirectInputW as windows_core::Interface>::IID || iid == &<IDirectInput2W as windows_core::Interface>::IID
    }
}
pub trait IDirectInput8A_Impl: Sized {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8A>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput8A {}
impl IDirectInput8A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput8A_Vtbl
    where
        Identity: IDirectInput8A_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::EnumDevicesBySemantics(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8A_Impl::ConfigureDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            FindDevice: FindDevice::<Identity, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput8A as windows_core::Interface>::IID
    }
}
pub trait IDirectInput8W_Impl: Sized {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8W>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInput8W {}
impl IDirectInput8W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInput8W_Vtbl
    where
        Identity: IDirectInput8W_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::EnumDevicesBySemantics(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInput8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInput8W_Impl::ConfigureDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            FindDevice: FindDevice::<Identity, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput8W as windows_core::Interface>::IID
    }
}
pub trait IDirectInputA_Impl: Sized {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceA>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputA {}
impl IDirectInputA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputA_Vtbl
    where
        Identity: IDirectInputA_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputA_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputA_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputA_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputA_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputA_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputA as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice2A_Impl: Sized + IDirectInputDeviceA_Impl {
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice2A {}
impl IDirectInputDevice2A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice2A_Vtbl
    where
        Identity: IDirectInputDevice2A_Impl,
    {
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::Poll(this).into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2A_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDeviceA_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice2A as windows_core::Interface>::IID || iid == &<IDirectInputDeviceA as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice2W_Impl: Sized + IDirectInputDeviceW_Impl {
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice2W {}
impl IDirectInputDevice2W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice2W_Vtbl
    where
        Identity: IDirectInputDevice2W_Impl,
    {
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::Poll(this).into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice2W_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDeviceW_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice2W as windows_core::Interface>::IID || iid == &<IDirectInputDeviceW as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice7A_Impl: Sized + IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice7A {}
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice7A_Vtbl
    where
        Identity: IDirectInputDevice7A_Impl,
    {
        unsafe extern "system" fn EnumEffectsInFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice7A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice7A_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice7A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice7A_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDevice2A_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as windows_core::Interface>::IID || iid == &<IDirectInputDeviceA as windows_core::Interface>::IID || iid == &<IDirectInputDevice2A as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice7W_Impl: Sized + IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice7W {}
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice7W_Vtbl
    where
        Identity: IDirectInputDevice7W_Impl,
    {
        unsafe extern "system" fn EnumEffectsInFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice7W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice7W_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice7W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice7W_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDevice2W_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as windows_core::Interface>::IID || iid == &<IDirectInputDeviceW as windows_core::Interface>::IID || iid == &<IDirectInputDevice2W as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice8A_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &windows_core::PCSTR, param2: u32) -> windows_core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &windows_core::PCSTR, param2: u32) -> windows_core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice8A {}
impl IDirectInputDevice8A_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice8A_Vtbl
    where
        Identity: IDirectInputDevice8A_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::Poll(this).into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8A_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8A_Impl::GetImageInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, OFFSET>,
            SetActionMap: SetActionMap::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice8A as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDevice8W_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &windows_core::PCWSTR, param2: u32) -> windows_core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &windows_core::PCWSTR, param2: u32) -> windows_core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDevice8W {}
impl IDirectInputDevice8W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDevice8W_Vtbl
    where
        Identity: IDirectInputDevice8W_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::Poll(this).into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::HRESULT
        where
            Identity: IDirectInputDevice8W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDevice8W_Impl::GetImageInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, OFFSET>,
            SetActionMap: SetActionMap::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice8W as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDeviceA_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDeviceA {}
impl IDirectInputDeviceA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDeviceA_Vtbl
    where
        Identity: IDirectInputDeviceA_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceA_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDeviceA as windows_core::Interface>::IID
    }
}
pub trait IDirectInputDeviceW_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputDeviceW {}
impl IDirectInputDeviceW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputDeviceW_Vtbl
    where
        Identity: IDirectInputDeviceW_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputDeviceW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputDeviceW_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDeviceW as windows_core::Interface>::IID
    }
}
pub trait IDirectInputEffect_Impl: Sized {
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetEffectGuid(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()>;
    fn SetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()>;
    fn Start(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetEffectStatus(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn Download(&self) -> windows_core::Result<()>;
    fn Unload(&self) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputEffect {}
impl IDirectInputEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputEffect_Vtbl
    where
        Identity: IDirectInputEffect_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::GetEffectGuid(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::GetParameters(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::SetParameters(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Start(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Stop(this).into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::GetEffectStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Download(this).into()
        }
        unsafe extern "system" fn Unload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Unload(this).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffect_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetEffectGuid: GetEffectGuid::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputEffect as windows_core::Interface>::IID
    }
}
pub trait IDirectInputEffectDriver_Impl: Sized {
    fn DeviceID(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersions(&self, param0: *mut DIDRIVERVERSIONS) -> windows_core::Result<()>;
    fn Escape(&self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn SetGain(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: u32, param1: *mut DIDEVICESTATE) -> windows_core::Result<()>;
    fn DownloadEffect(&self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> windows_core::Result<()>;
    fn DestroyEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn StartEffect(&self, param0: u32, param1: u32, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn StopEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn GetEffectStatus(&self, param0: u32, param1: u32, param2: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputEffectDriver {}
impl IDirectInputEffectDriver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputEffectDriver_Vtbl
    where
        Identity: IDirectInputEffectDriver_Impl,
    {
        unsafe extern "system" fn DeviceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::DeviceID(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetVersions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::GetVersions(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::Escape(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetGain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::SetGain(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn DownloadEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::DownloadEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DestroyEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::DestroyEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::StartEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn StopEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::StopEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputEffectDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputEffectDriver_Impl::GetEffectStatus(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceID: DeviceID::<Identity, OFFSET>,
            GetVersions: GetVersions::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            SetGain: SetGain::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            DownloadEffect: DownloadEffect::<Identity, OFFSET>,
            DestroyEffect: DestroyEffect::<Identity, OFFSET>,
            StartEffect: StartEffect::<Identity, OFFSET>,
            StopEffect: StopEffect::<Identity, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputEffectDriver as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IDirectInputJoyConfig_Impl: Sized {
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SendNotify(&self) -> windows_core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn SetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn DeleteType(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OpenTypeKey(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn OpenConfigKey(&self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IDirectInputJoyConfig {}
#[cfg(feature = "Win32_System_Registry")]
impl IDirectInputJoyConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputJoyConfig_Vtbl
    where
        Identity: IDirectInputJoyConfig_Impl,
    {
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::SendNotify(this).into()
        }
        unsafe extern "system" fn EnumTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::EnumTypes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::GetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::SetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::DeleteType(this, core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::GetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::SetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::DeleteConfig(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::GetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::SetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::AddNewHardware(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::OpenTypeKey(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenConfigKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig_Impl::OpenConfigKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SendNotify: SendNotify::<Identity, OFFSET>,
            EnumTypes: EnumTypes::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, OFFSET>,
            DeleteType: DeleteType::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
            SetConfig: SetConfig::<Identity, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, OFFSET>,
            GetUserValues: GetUserValues::<Identity, OFFSET>,
            SetUserValues: SetUserValues::<Identity, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, OFFSET>,
            OpenConfigKey: OpenConfigKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IDirectInputJoyConfig8_Impl: Sized {
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SendNotify(&self) -> windows_core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn SetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteType(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OpenTypeKey(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn OpenAppStatusKey(&self, param0: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IDirectInputJoyConfig8 {}
#[cfg(feature = "Win32_System_Registry")]
impl IDirectInputJoyConfig8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputJoyConfig8_Vtbl
    where
        Identity: IDirectInputJoyConfig8_Impl,
    {
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::Acquire(this).into()
        }
        unsafe extern "system" fn Unacquire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::Unacquire(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::SendNotify(this).into()
        }
        unsafe extern "system" fn EnumTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::EnumTypes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::GetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::SetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::DeleteType(this, core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::GetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::SetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::DeleteConfig(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::GetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::SetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::AddNewHardware(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::OpenTypeKey(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenAppStatusKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT
        where
            Identity: IDirectInputJoyConfig8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputJoyConfig8_Impl::OpenAppStatusKey(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SendNotify: SendNotify::<Identity, OFFSET>,
            EnumTypes: EnumTypes::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, OFFSET>,
            DeleteType: DeleteType::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
            SetConfig: SetConfig::<Identity, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, OFFSET>,
            GetUserValues: GetUserValues::<Identity, OFFSET>,
            SetUserValues: SetUserValues::<Identity, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, OFFSET>,
            OpenAppStatusKey: OpenAppStatusKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig8 as windows_core::Interface>::IID
    }
}
pub trait IDirectInputW_Impl: Sized {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceW>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectInputW {}
impl IDirectInputW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectInputW_Vtbl
    where
        Identity: IDirectInputW_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectInputW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputW_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputW_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectInputW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputW_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputW_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectInputW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectInputW_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputW as windows_core::Interface>::IID
    }
}