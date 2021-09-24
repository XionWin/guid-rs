# guid-rs
---
Guid is a little demonstration program for how to drive bare metal graphics without a compositor like X11, wayland or similar, using DRM/KMS (kernel mode setting), GBM (graphics buffer manager) and EGL for rendering content using OpenGL or OpenGL ES.Cancel changes

Library dependecies: \b
1. **libc**: involved c languge data format (like: uint_32t c_void etc.) for easier life.
2. **bitflags**: enhance rust to suport bit opration on a structured data, very similar with the Flag enum in C#

---

![DRM/KMS diagram](https://github.com/XionWin/guid-rs/blob/main/resource/Linux_graphics_drivers_DRI_Wayland.svg.png?raw=true)
