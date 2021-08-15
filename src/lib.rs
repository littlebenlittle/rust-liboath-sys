#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    use crate::*;
    use libc::time_t;
    use std::ffi::CString;

    #[test]
    fn it_generates_a_totp() {
        let key = "\x31\x32\x33\x34\x35\x36\x37\x38\x39".to_string();
        let secret_length = key.len() as u64;
        let secret = CString::new(key).expect("key could not be converted to CString");
        let secret_ptr = secret.as_ptr();
        let now = 54321 as time_t;
        let time_step_size = 30 as u32;
        let start_offset = 0 as i64;
        let digits = 6 as u32;
        let mut output_otp = 0 as i8;
        let ref mut output_otp_ptr = output_otp;
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
        }
        assert_eq!(output_otp, 49 as i8);
    }

}
