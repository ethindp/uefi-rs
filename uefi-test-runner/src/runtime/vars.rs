use log::info;
use uefi::prelude::*;
use uefi::table::runtime::{VariableAttributes, VariableVendor};
use uefi::{CStr16, Guid};

fn test_variables(rt: &RuntimeServices) {
    let mut buf = [0; 14];
    let name = CStr16::from_str_with_buf("UefiRsTestVar", &mut buf).unwrap();
    let test_value = b"TestValue";
    let test_attrs = VariableAttributes::BOOTSERVICE_ACCESS | VariableAttributes::RUNTIME_ACCESS;

    // Arbitrary GUID generated for this test.
    let vendor = VariableVendor(Guid::from_values(
        0x9baf21cf,
        0xe187,
        0x497e,
        0xae77,
        0x5bd8b0e09703,
    ));

    info!("Testing set_variable");
    rt.set_variable(name, &vendor, test_attrs, test_value)
        .expect_success("failed to set variable");

    info!("Testing get_variable_size");
    let size = rt
        .get_variable_size(name, &vendor)
        .expect_success("failed to get variable size");
    assert_eq!(size, test_value.len());

    info!("Testing get_variable");
    let mut buf = [0u8; 9];
    let (data, attrs) = rt
        .get_variable(name, &vendor, &mut buf)
        .expect_success("failed to get variable");
    assert_eq!(data, test_value);
    assert_eq!(attrs, test_attrs);

    info!("Testing variable_keys");
    let variable_keys = rt
        .variable_keys()
        .expect_success("failed to get variable keys");
    info!("Found {} variables", variable_keys.len());
    // There are likely a bunch of variables, only print out the first one
    // during the test to avoid spamming the log.
    if let Some(key) = variable_keys.first() {
        info!("First variable: {}", key);
    }
}

pub fn test(rt: &RuntimeServices) {
    test_variables(rt);
}
