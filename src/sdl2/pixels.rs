use std::rand;

pub mod ll {
    use std::libc::{c_int, uint8_t, uint32_t};

    //SDL_pixels.h
    pub struct SDL_Color {
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
        a: uint8_t,
    }

    pub struct SDL_Palette {
        ncolors: c_int,
        colors: *SDL_Color,
        version: uint32_t,
        refcount: c_int
    }

    pub struct SDL_PixelFormat {
        format: SDL_PixelFormatFlag,
        palette: *SDL_Palette,
        BitsPerPixel: uint8_t,
        BytesPerPixel: uint8_t,
        padding: [uint8_t, ..2],
        Rmask: uint8_t,
        Gmask: uint8_t,
        Bmask: uint8_t,
        Amask: uint8_t,
        Rloss: uint8_t,
        Gloss: uint8_t,
        Bloss: uint8_t,
        Aloss: uint8_t,
        Rshift: uint8_t,
        Gshift: uint8_t,
        Bshift: uint8_t,
        Ashift: uint8_t,
        refcount: c_int,
        next: *SDL_PixelFormat
    }

    pub type SDL_PixelFormatFlag = uint32_t;
    pub static SDL_PIXELFORMAT_UNKNOWN: SDL_PixelFormatFlag = 0x0;
    pub static SDL_PIXELFORMAT_INDEX1LSB: SDL_PixelFormatFlag = 0x11100100;
    pub static SDL_PIXELFORMAT_INDEX1MSB: SDL_PixelFormatFlag = 0x11200100;
    pub static SDL_PIXELFORMAT_INDEX4LSB: SDL_PixelFormatFlag = 0x12100400;
    pub static SDL_PIXELFORMAT_INDEX4MSB: SDL_PixelFormatFlag = 0x12200400;
    pub static SDL_PIXELFORMAT_INDEX8: SDL_PixelFormatFlag = 0x13000801;
    pub static SDL_PIXELFORMAT_RGB332: SDL_PixelFormatFlag = 0x14110801;
    pub static SDL_PIXELFORMAT_RGB444: SDL_PixelFormatFlag = 0x15120c02;
    pub static SDL_PIXELFORMAT_RGB555: SDL_PixelFormatFlag = 0x15130f02;
    pub static SDL_PIXELFORMAT_BGR555: SDL_PixelFormatFlag = 0x15530f02;
    pub static SDL_PIXELFORMAT_ARGB4444: SDL_PixelFormatFlag = 0x15321002;
    pub static SDL_PIXELFORMAT_RGBA4444: SDL_PixelFormatFlag = 0x15421002;
    pub static SDL_PIXELFORMAT_ABGR4444: SDL_PixelFormatFlag = 0x15721002;
    pub static SDL_PIXELFORMAT_BGRA4444: SDL_PixelFormatFlag = 0x15821002;
    pub static SDL_PIXELFORMAT_ARGB1555: SDL_PixelFormatFlag = 0x15331002;
    pub static SDL_PIXELFORMAT_RGBA5551: SDL_PixelFormatFlag = 0x15441002;
    pub static SDL_PIXELFORMAT_ABGR1555: SDL_PixelFormatFlag = 0x15731002;
    pub static SDL_PIXELFORMAT_BGRA5551: SDL_PixelFormatFlag = 0x15841002;
    pub static SDL_PIXELFORMAT_RGB565: SDL_PixelFormatFlag = 0x15151002;
    pub static SDL_PIXELFORMAT_BGR565: SDL_PixelFormatFlag = 0x15551002;
    pub static SDL_PIXELFORMAT_RGB24: SDL_PixelFormatFlag = 0x17101803;
    pub static SDL_PIXELFORMAT_BGR24: SDL_PixelFormatFlag = 0x17401803;
    pub static SDL_PIXELFORMAT_RGB888: SDL_PixelFormatFlag = 0x16161804;
    pub static SDL_PIXELFORMAT_RGBX8888: SDL_PixelFormatFlag = 0x16261804;
    pub static SDL_PIXELFORMAT_BGR888: SDL_PixelFormatFlag = 0x16561804;
    pub static SDL_PIXELFORMAT_BGRX8888: SDL_PixelFormatFlag = 0x16661804;
    pub static SDL_PIXELFORMAT_ARGB8888: SDL_PixelFormatFlag = 0x16362004;
    pub static SDL_PIXELFORMAT_RGBA8888: SDL_PixelFormatFlag = 0x16462004;
    pub static SDL_PIXELFORMAT_ABGR8888: SDL_PixelFormatFlag = 0x16762004;
    pub static SDL_PIXELFORMAT_BGRA8888: SDL_PixelFormatFlag = 0x16862004;
    pub static SDL_PIXELFORMAT_ARGB2101010: SDL_PixelFormatFlag = 0x16372004;
    pub static SDL_PIXELFORMAT_YV12: SDL_PixelFormatFlag = 0x32315659;
    pub static SDL_PIXELFORMAT_IYUV: SDL_PixelFormatFlag = 0x56555949;
    pub static SDL_PIXELFORMAT_YUY2: SDL_PixelFormatFlag = 0x32595559;
    pub static SDL_PIXELFORMAT_UYVY: SDL_PixelFormatFlag = 0x59565955;
    pub static SDL_PIXELFORMAT_YVYU: SDL_PixelFormatFlag = 0x55595659;
}
#[deriving(Eq)]
pub struct Palette {
    raw: *ll::SDL_Palette
}

#[deriving(Eq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

#[deriving(Eq, FromPrimitive)]
pub enum PixelFormatFlag {
    Unknown = ll::SDL_PIXELFORMAT_UNKNOWN as int,
    Index1LSB = ll::SDL_PIXELFORMAT_INDEX1LSB as int,
    Index1MSB = ll::SDL_PIXELFORMAT_INDEX1MSB as int,
    Index4LSB = ll::SDL_PIXELFORMAT_INDEX4LSB as int,
    Index4MSB = ll::SDL_PIXELFORMAT_INDEX4MSB as int,
    Index8 = ll::SDL_PIXELFORMAT_INDEX8 as int,
    RGB332 = ll::SDL_PIXELFORMAT_RGB332 as int,
    RGB444 = ll::SDL_PIXELFORMAT_RGB444 as int,
    RGB555 = ll::SDL_PIXELFORMAT_RGB555 as int,
    BGR555 = ll::SDL_PIXELFORMAT_BGR555 as int,
    ARGB4444 = ll::SDL_PIXELFORMAT_ARGB4444 as int,
    RGBA4444 = ll::SDL_PIXELFORMAT_RGBA4444 as int,
    ABGR4444 = ll::SDL_PIXELFORMAT_ABGR4444 as int,
    BGRA4444 = ll::SDL_PIXELFORMAT_BGRA4444 as int,
    ARGB1555 = ll::SDL_PIXELFORMAT_ARGB1555 as int,
    RGBA5551 = ll::SDL_PIXELFORMAT_RGBA5551 as int,
    ABGR1555 = ll::SDL_PIXELFORMAT_ABGR1555 as int,
    BGRA5551 = ll::SDL_PIXELFORMAT_BGRA5551 as int,
    RGB565 = ll::SDL_PIXELFORMAT_RGB565 as int,
    BGR565 = ll::SDL_PIXELFORMAT_BGR565 as int,
    RGB24 = ll::SDL_PIXELFORMAT_RGB24 as int,
    BGR24 = ll::SDL_PIXELFORMAT_BGR24 as int,
    RGB888 = ll::SDL_PIXELFORMAT_RGB888 as int,
    RGBX8888 = ll::SDL_PIXELFORMAT_RGBX8888 as int,
    BGR888 = ll::SDL_PIXELFORMAT_BGR888 as int,
    BGRX8888 = ll::SDL_PIXELFORMAT_BGRX8888 as int,
    ARGB8888 = ll::SDL_PIXELFORMAT_ARGB8888 as int,
    RGBA8888 = ll::SDL_PIXELFORMAT_RGBA8888 as int,
    ABGR8888 = ll::SDL_PIXELFORMAT_ABGR8888 as int,
    BGRA8888 = ll::SDL_PIXELFORMAT_BGRA8888 as int,
    ARGB2101010 = ll::SDL_PIXELFORMAT_ARGB2101010 as int,
    YV12 = ll::SDL_PIXELFORMAT_YV12 as int,
    IYUV = ll::SDL_PIXELFORMAT_IYUV as int,
    YUY2 = ll::SDL_PIXELFORMAT_YUY2 as int,
    UYVY = ll::SDL_PIXELFORMAT_UYVY as int,
    YVYU = ll::SDL_PIXELFORMAT_YVYU as int
}
