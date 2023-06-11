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
        size: *const c_int,
    ) -> *const c_uchar;

    fn AquesTalk_FreeWave(wav: *const c_uchar) -> c_void;
}

pub fn synthe(text: String) -> Result<Vec<u8>> {
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
    let size = 0;
    let wave = unsafe { AquesTalk_Synthe_Utf8(&voice, text.as_ptr(), &size as *const i32) };
    println!("size: {}", size);
    unsafe { AquesTalk_FreeWave(wave) };
    let wave = unsafe { std::slice::from_raw_parts(wave, 48000) };
    Ok(wave.to_vec())
}