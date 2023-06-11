use libc::{c_char, c_int, c_uchar, c_void};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[repr(C)]
struct AqtkVoice {
    bas: c_int,
    spd: c_int,
    vol: c_int,
    pit: c_int,
    acc: c_int,
    lmd: c_int,
    fsc: c_int,
}

extern "C" {
    fn AquesTalk_Synthe_Utf8(
        pParam: *const AqtkVoice,
        char: *const c_char,
        size: *mut c_int,
    ) -> *mut c_uchar;

    fn AquesTalk_FreeWave(wav: *const c_uchar) -> c_void;
}

pub fn synthe(text: String) -> Result<Vec<mut u8>> {
    let voice = AqtkVoice {
        bas: 0,
        spd: 100,
        vol: 100,
        pit: 100,
        acc: 100,
        lmd: 100,
        fsc: 100,
    };
    let text = std::ffi::CString::new(text)?;
    let mut size = 0;
    let mut wave = unsafe { AquesTalk_Synthe_Utf8(&voice, text.as_ptr(), &mut size) };
    println!("size: {}", size);
    if size == 105 {
        return Err("error".into());
    }
    unsafe { AquesTalk_FreeWave(wave) };
    let wave = unsafe { std::vec::Vec::from_raw_parts(&mut wave, size.try_into().unwrap() as usize, size.try_into().unwrap() as usize) };
    Ok(wave)
}
