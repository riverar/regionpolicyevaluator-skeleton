use bindings::*;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::mem::*;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::WinRT::{IActivationFactory, IActivationFactory_Impl},
    },
};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    _name: ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    let factory: IActivationFactory = ClassFactory {
        log: OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}\\component.log", env!("Temp")))
            .unwrap(),
    }
    .into();
    *result = std::mem::transmute(factory);
    S_OK
}

#[implement(IRegionPolicyEvaluatorStatics, IActivationFactory, TrustLevel = Partial)]
struct ClassFactory {
    log: File,
}

impl IRegionPolicyEvaluatorStatics_Impl for ClassFactory {
    fn EvaluatePolicyState(
        &self,
        policy: &windows_core::GUID,
    ) -> windows_core::Result<PolicyState> {
        writeln!(&self.log, "EvaluatePolicyState({:?})", policy).unwrap();
        Ok(PolicyState::Enabled)
    }

    fn ForceEvaluatePolicyState(
        &self,
        policy: &windows_core::GUID,
    ) -> windows_core::Result<PolicyState> {
        writeln!(&self.log, "ForceEvaluatePolicyState({:?})", policy).unwrap();
        Ok(PolicyState::Enabled)
    }
}

impl IActivationFactory_Impl for ClassFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}
