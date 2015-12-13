extern crate libc;
use std::ffi;
use std::ptr;

pub mod plugin;
pub mod test_plugin;
use plugin::Plugin;
use test_plugin::TestPlugin;

extern crate xplm;

type PluginType = TestPlugin;
type PluginPtr = *mut PluginType;
// The plugin
static mut PLUGIN: PluginPtr = 0 as PluginPtr;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginStart(outName: *mut libc::c_char, outSig: *mut libc::c_char,
    outDescription: *mut libc::c_char) -> libc::c_int
{
    // Enable native paths
    let feature_result = xplm::features::set_feature_enabled("XPLM_USE_NATIVE_PATHS", true);
    if feature_result.is_err() {
        xplm::debug("Failed to enable XPLM_USE_NATIVE_PATHS\n");
    }

    // Create the plugin, temporarily, on the stack
    let plugin_option = PluginType::start();

    match plugin_option {
        Some(plugin) => {
            // Allocate storage
            PLUGIN = Box::into_raw(Box::new(plugin));

            let info = (*PLUGIN).info();

            match ffi::CString::new(info.name).ok() {
                Some(name) => libc::strcpy(outName, name.as_ptr()),
                None => libc::strcpy(outName, b"<invalid>".as_ptr() as *const libc::c_char),
            };
            match ffi::CString::new(info.signature).ok() {
                Some(signature) => libc::strcpy(outSig, signature.as_ptr()),
                None => libc::strcpy(outSig, b"<invalid>".as_ptr() as *const libc::c_char),
            };
            match ffi::CString::new(info.description).ok() {
                Some(description) => libc::strcpy(outDescription, description.as_ptr()),
                None => libc::strcpy(outDescription, b"<invalid>".as_ptr() as *const libc::c_char),
            };

            // Success
            1
        },
        None => {
            xplm::debug("Plugin returned None from start method\n");
            // Return failure
            0
        },
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginStop() {
    (*PLUGIN).stop();
    // Free plugin
    let plugin_box = Box::from_raw(PLUGIN);
    drop(plugin_box);
    PLUGIN = ptr::null_mut();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginEnable() {
    (*PLUGIN).enable();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginDisable() {
    (*PLUGIN).disable();
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub unsafe extern "C" fn XPluginReceiveMessage(inFrom: libc::c_int, inMessage: libc::c_int,
    inParam: *mut libc::c_void)
{
    // Nothing
}
