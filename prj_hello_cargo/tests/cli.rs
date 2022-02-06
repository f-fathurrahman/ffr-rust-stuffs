use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("prj_hello_cargo").unwrap();
    cmd.assert().success();
}
