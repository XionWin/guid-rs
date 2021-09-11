bitflags! {
    #[repr(C)]
    pub struct DrmModeType: u32 {
        const BUILT_IN = 1<<0;
        const COCK_C = (1<<1 | 1<<0);
        const CRTC_C = (1<<2 | 1<<0);
        const PREFERRED = 1<<3;
        const DEFAULT = 1<<4;
        const USER_DEFAULT = 1<<5;
        const DRIVER = 1<<6;
    }
}