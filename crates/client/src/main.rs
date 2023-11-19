use bindings::*;
use windows::{
    core::GUID,
    Win32::System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        RoInitialize(RO_INIT_MULTITHREADED)?;

        let state = RegionPolicyEvaluator::EvaluatePolicyState(GUID::from_u128(
            0x6033b294_79ff_47eb_bbde_97b4c6479217,
        ))?;
        dbg!(state);

        let state = RegionPolicyEvaluator::ForceEvaluatePolicyState(GUID::from_u128(
            0x6033b294_79ff_47eb_bbde_97b4c6479217,
        ))?;
        dbg!(state);

        Ok(())
    }
}
