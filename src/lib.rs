#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings-core.rs"));

#[cfg(feature = "studio")]
include!(concat!(env!("OUT_DIR"), "/bindings-studio.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn fmod_test_create_release() {
        let mut system: *mut FMOD_SYSTEM = ptr::null_mut();
        let result = unsafe { FMOD_System_Create(&mut system) };
        assert_eq!(result, FMOD_RESULT_FMOD_OK);
        let result = unsafe { FMOD_System_Release(system) };
        assert_eq!(result, FMOD_RESULT_FMOD_OK);
    }

    #[cfg(feature = "studio")]
    #[test]
    fn fmod_studio_test_create_release() {
        let mut system: *mut FMOD_STUDIO_SYSTEM = ptr::null_mut();
        let result = unsafe { FMOD_Studio_System_Create(&mut system, FMOD_VERSION) };
        assert_eq!(result, FMOD_RESULT_FMOD_OK);
        let result = unsafe { FMOD_Studio_System_Release(system) };
        assert_eq!(result, FMOD_RESULT_FMOD_OK);
    }
}
