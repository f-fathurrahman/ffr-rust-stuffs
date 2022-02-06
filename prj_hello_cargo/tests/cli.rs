use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new("prj_hello_cargo");
    let res = cmd.output();
    assert!(res.is_ok());
}
