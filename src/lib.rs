#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    use crate::*;
    use libc::{time_t, c_char, c_uint};
    use std::ffi::{CStr, CString};

    #[test]
    fn it_generates_a_totp() {
        let key = "\x31\x32\x33\x34\x35\x36\x37\x38\x39".to_string();
        let secret_length = key.len() as u64;
        let secret = CString::new(key).expect("key could not be converted to CString");
        let secret_ptr = secret.as_ptr();
        let now = 54321 as time_t;
        let time_step_size = 30 as c_uint;
        let start_offset = 0 as time_t;
        let digits = 6 as c_uint;
        let mut output_otp: Vec<c_char> = vec!(0; digits as usize + 1);
        let output_otp_ptr = output_otp.as_mut_ptr();
        let otp: CString;
        unsafe {
            oath_totp_generate(
                secret_ptr,
                secret_length,
                now,
                time_step_size,
                start_offset,
                digits,
                output_otp_ptr,
            );
            otp = CStr::from_ptr(output_otp_ptr).to_owned();
        }
        let expects = CString::new("115325").unwrap();
        assert_eq!(otp, expects);
    }

}
