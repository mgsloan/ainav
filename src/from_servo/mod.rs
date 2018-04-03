// Why did the font rendering get split out of servo instead of using
// freetype-rs?
//
// Well, I wanted to use a highlevel wrapper for fontconfig, and the only option
// I could find was to pull code out of servo. That code uses
// "servo-fontconfig", which depends on "servo-fontconfig-sys", which depends on
// "servo-freetype-sys". The "freetype-rs" crate I'd like to use depends on
// "freetype-sys". Can't have 2 linking the same native lib.

// mod fast_hash_map;
// mod font;
mod font_list;
// mod font_types;

/*
extern crate libc;

#[macro_use]
extern crate log;
*/

/* font_list::for_each_variation("Hack", |path| {
println!("{}", path);
    });
    */


// use font;

/*

// TODO: package this up in some internal struct.

pub fn init_fontconfig() -> *mut FcConfig {
    unsafe {
        return FcInitLoadConfigAndFonts();
    }
}

pub fn find_font(fc: *mut FcConfig) {
    let mut pat
        : *mut FcPattern
        = FcNameParse((*b"Arial\0").as_ptr());
    FcConfigSubstitute(config, pat, _FcMatchKind::FcMatchPattern);
    FcDefaultSubstitute(pat);
    let mut fontFile : *mut u8;
    let mut result : _FcResult;
    let mut font
        : *mut FcPattern
        = FcFontMatch(config,pat,&mut result as (*mut _FcResult));
    /*
    if !font.is_null() {
        let mut file
            : *mut u8
            = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        if FcPatternGetString(
            font as (*const FcPattern),
            (*b"file\0").as_ptr(),
            0i32,
            &mut file as (*mut *mut u8)
        ) as (i32) == _FcResult::FcResultMatch as (i32) {
            fontFile = file;
            print!("{}",fontFile);
        }
    }
    */
    FcPatternDestroy(pat);
}
*/
