#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetResources(fd: libc::c_int) -> *const crate::resources::DrmResources;
}