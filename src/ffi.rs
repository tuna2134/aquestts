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
    let mut size = 0;
    let wave = unsafe { AquesTalk_Synthe_Utf8(&voice, text.as_ptr(), &mut size) };
    println!("size: {}", size);
    if size == 105 {
        return Err("error".into());
    }
    let wav: Vec<u8> = unsafe { std::vec::Vec::from_raw_parts(wave, size as usize, size as usize) };
    println!("Free!");
    Ok(wav)
}
