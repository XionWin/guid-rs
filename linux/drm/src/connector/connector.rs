use crate::ModeInfo;

#[derive(Debug)]
pub struct Connector {
    handle: *const crate::ffi::DrmConnector,
    connector_id: libc::c_uint,
    encoder_id: libc::c_uint,
    connector_type: crate::def::ConnectorType,
    connector_type_id: libc::c_uint,
    connection_status: crate::def::ConnectionStatus,

    mm_width: libc::c_uint,
    mm_height: libc::c_uint,
    
    subpixel: crate::def::SubPixel,

    pub(crate) modes: Vec<ModeInfo>,

    // count_props: libc::c_int,
    // props: *const libc::c_uint,
    // prop_values: *const libc::c_ulong,

    // count_encoders: libc::c_int,
    // encoders: *const libc::c_uint,
}

impl Connector {
    pub fn new(c: &crate::ffi::DrmConnector) -> Self {
        Self {
            handle: c as *const crate::ffi::DrmConnector,
            connector_id : c.connector_id,
            encoder_id : c.encoder_id,
            connector_type : c.connector_type,
            connector_type_id : c.connector_type_id,
            connection_status : c.connection,
            mm_width : c.mm_width,
            mm_height : c.mm_height,
            subpixel : c.subpixel,
            
            modes : get_modes(c)
        }
    }

    pub fn get_id(&self) -> libc::c_uint {
        self.connector_id
    }

    pub fn get_connection_status(&self) -> crate::def::ConnectionStatus {
        self.connection_status
    }

    pub fn get_encoder_id(&self) -> libc::c_uint {
        self.encoder_id
    }
}

// impl Drop for Connector {
//     fn drop(&mut self) {
//         unsafe {
//             crate::ffi::drmModeFreeConnector(self.handle);
//             println!("Connector: {:?} droped", self.handle);
//         }
//     }
// }

fn get_modes(c: &crate::ffi::DrmConnector) -> Vec<crate::ModeInfo> {
    unsafe {std::slice::from_raw_parts(c.modes, c.count_modes as usize)}.iter().map(|x| {
        crate::ModeInfo::new(x)
    }).collect::<Vec<crate::ModeInfo>>()
}