use rexl::io::{base_name, ext_name};

#[test]
fn test_base_name() {
    assert_eq!(base_name("/etc/mysql/"), "mysql");
    assert_eq!(base_name("/"), "/");
    assert_eq!(base_name("/etc/mysql/my.ini"), "my.ini");
}

#[test]
fn test_ext_name() {
    assert_eq!(ext_name("/etc/mysql/my.ini"), "ini");
}
