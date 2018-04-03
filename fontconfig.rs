extern {
    fn FcConfigSubstitute(
        config : *mut _FcConfig, p : *mut _FcPattern, kind : _FcMatchKind
    ) -> i32;
    fn FcDefaultSubstitute(pattern : *mut _FcPattern);
    fn FcFontMatch(
        config : *mut _FcConfig,
        p : *mut _FcPattern,
        result : *mut _FcResult
    ) -> *mut _FcPattern;
    fn FcInitLoadConfigAndFonts() -> *mut _FcConfig;
    fn FcNameParse(name : *const u8) -> *mut _FcPattern;
    fn FcPatternDestroy(p : *mut _FcPattern);
    fn FcPatternGetString(
        p : *const _FcPattern,
        object : *const u8,
        n : i32,
        s : *mut *mut u8
    ) -> _FcResult;
    fn printf(__format : *const u8, ...) -> i32;
}

enum _FcConfig {
}

enum _FcPattern {
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum _FcMatchKind {
    FcMatchPattern,
    FcMatchFont,
    FcMatchScan,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum _FcResult {
    FcResultMatch,
    FcResultNoMatch,
    FcResultTypeMismatch,
    FcResultNoId,
    FcResultOutOfMemory,
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    let mut config : *mut _FcConfig = FcInitLoadConfigAndFonts();
    let mut pat
        : *mut _FcPattern
        = FcNameParse((*b"Arial\0").as_ptr());
    FcConfigSubstitute(config,pat,_FcMatchKind::FcMatchPattern);
    FcDefaultSubstitute(pat);
    let mut fontFile : *mut u8;
    let mut result : _FcResult;
    let mut font
        : *mut _FcPattern
        = FcFontMatch(config,pat,&mut result as (*mut _FcResult));
    if !font.is_null() {
        let mut file
            : *mut u8
            = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        if FcPatternGetString(
               font as (*const _FcPattern),
               (*b"file\0").as_ptr(),
               0i32,
               &mut file as (*mut *mut u8)
           ) as (i32) == _FcResult::FcResultMatch as (i32) {
            fontFile = file;
            printf((*b"%s\n\0").as_ptr(),fontFile);
        }
    }
    FcPatternDestroy(pat);
    0i32
}
