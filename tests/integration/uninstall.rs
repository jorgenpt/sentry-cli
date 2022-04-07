use crate::integration::register_test;

#[test]
fn command_uninstall() {
    #[cfg(not(windows))]
    let _t = register_test("uninstall/uninstall.trycmd");
    #[cfg(windows)]
    let _t = register_test("uninstall/uninstall-windows.trycmd");
}