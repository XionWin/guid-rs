# GUID RS
---
Guid is a little demonstration program for driving bare metal graphics without a compositor like X11, Wayland, or similar, using DRM/KMS (kernel mode setting), GBM (graphics buffer manager), and EGL for rendering content using OpenGL or OpenGL ES.

Library dependencies:
1. **libc**: involved c language data format (like uint_32t c_void etc.) for an easier life when using FFI play with C libraries.
2. **bitflags**: enhance rust to support bit operation on structured data, very similar to the Flag enum in C#

---
## Tested on raspberry pi (Raspbian lite) with Waveshare AMOLED screen
![image_01](https://github.com/XionWin/guid-rs/blob/main/resources/image_01.jpg?raw=true)
![image_02](https://github.com/XionWin/guid-rs/blob/main/resources/image_02.jpg?raw=true)

## Memory useage is very low
![image_03](https://github.com/XionWin/guid-rs/blob/main/resources/image_03.jpg?raw=true)

---
![DRM/KMS diagram](https://github.com/XionWin/guid-rs/blob/main/resources/Linux_graphics_drivers_DRI_Wayland.svg.png?raw=true)
