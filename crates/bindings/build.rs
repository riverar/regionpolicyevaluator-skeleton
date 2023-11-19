use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=winmd/W.I.S.P.RegionPolicyEvaluator.idl");
    println!("cargo:rerun-if-changed=riddle.rsp");

    let mut command = Command::new("midlrt.exe");
    command.arg("@midlrt.rsp");

    if !command.status().unwrap().success() {
        panic!("MIDLRT failed.");
    }

    let status = Command::new("riddle")
        .arg("--etc")
        .arg("riddle.rsp")
        .status()
        .expect("Failed to run riddle. Is it installed?");

    assert!(status.code().unwrap() == 0);
}
