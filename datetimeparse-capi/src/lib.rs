use std::os::raw::c_int;

const PDT_SUCCESS: c_int = 0;
const PDT_PARSE_ERROR: c_int = 1;
const PDT_MALFORMED_STR: c_int = 2;

const ERR_MESSAGES: [&str; 3] = ["Success", "Parse error", "Malformed input string"];

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct pdt_precise_local_date_time {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub hour: c_int,
    pub minute: c_int,
    pub second: c_int,
    pub nanosecond: c_int,
}

#[no_mangle]
#[allow(unused_assignments)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn pdt_parse_rfc3339_datetime(
    inp: *const u8,
    inp_len: usize,
    out: *mut pdt_precise_local_date_time,
) -> c_int {
    let inp = unsafe { std::slice::from_raw_parts(inp, inp_len) };
    let Ok(inp) = std::str::from_utf8(inp) else {
        return PDT_MALFORMED_STR;
    };
    let Ok(dt) = datetimeparse::parse_rfc3339_datetime(inp) else {
        return PDT_PARSE_ERROR;
    };
    unsafe {
        (*out).year = dt.year.try_into().unwrap();
        (*out).month = dt.month.try_into().unwrap();
        (*out).day = dt.day.try_into().unwrap();
        (*out).hour = dt.hour.try_into().unwrap();
        (*out).minute = dt.minute.try_into().unwrap();
        (*out).second = dt.second.try_into().unwrap();
        (*out).nanosecond = dt.nanosecond.try_into().unwrap();
    }
    PDT_SUCCESS
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn pdt_perror(inp: *const u8, error: c_int) {
    unsafe {
        if !inp.is_null() && *inp != b'\0' {
            let len = libc::strlen(inp as *const libc::c_char);
            let inp = std::slice::from_raw_parts(inp, len);
            let inp = std::str::from_utf8(inp).unwrap();
            eprint!("{}: ", inp);
        }
    }
    eprintln!(
        "{}",
        ERR_MESSAGES.get(error as usize).unwrap_or(&"Unknown error")
    );
}
