use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

use esp_idf_sys::*;

static BLE_STARTED: AtomicBool = AtomicBool::new(false);
static BLE_CONNECTED: AtomicBool = AtomicBool::new(false);

/// Callback user (raw bytes)
static mut WRITE_CB: Option<fn(&[u8])> = None;

pub struct BleDriver;

impl BleDriver {
    pub fn new() -> Result<Self, EspError> {
        Ok(Self)
    }

    pub fn on_write(&mut self, cb: fn(&[u8])) {
        unsafe { WRITE_CB = Some(cb) }
    }

    pub fn start(&mut self) -> Result<(), EspError> {
        if BLE_STARTED.swap(true, Ordering::SeqCst) {
            return Ok(()); // already started
        }

        unsafe {
            esp_nimble_hci_and_controller_init();
            nimble_port_init();

            ble_svc_gap_init();
            ble_svc_gatt_init();

            ble_svc_gap_device_name_set(b"ESP32-BLE\0".as_ptr() as _);

            ble_gatts_count_cfg(GATT_DEFS.as_ptr());
            ble_gatts_add_svcs(GATT_DEFS.as_ptr());

            ble_gap_adv_set_fields(&ADV_FIELDS);
            ble_gap_adv_start(
                BLE_OWN_ADDR_PUBLIC,
                ptr::null(),
                BLE_HS_FOREVER as _,
                &ADV_PARAMS,
                Some(gap_event),
                ptr::null_mut(),
            );

            nimble_port_freertos_init(Some(ble_host_task));
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        BLE_CONNECTED.load(Ordering::SeqCst)
    }

    pub fn stop(&mut self) -> Result<(), EspError> {
        unsafe {
            ble_gap_adv_stop();
            nimble_port_stop();
            nimble_port_deinit();
        }
        BLE_STARTED.store(false, Ordering::SeqCst);
        Ok(())
    }
}

/* ===================== NIMBLE CALLBACKS ===================== */

extern "C" fn ble_host_task(_: *mut c_void) {
    unsafe {
        nimble_port_run();
        nimble_port_freertos_deinit();
    }
}

extern "C" fn gap_event(
    event: *mut ble_gap_event,
    _: *mut c_void,
) -> i32 {
    unsafe {
        match (*event).type_ {
            BLE_GAP_EVENT_CONNECT => {
                BLE_CONNECTED.store(true, Ordering::SeqCst);
            }
            BLE_GAP_EVENT_DISCONNECT => {
                BLE_CONNECTED.store(false, Ordering::SeqCst);
            }
            _ => {}
        }
    }
    0
}

extern "C" fn gatt_access(
    _: u16,
    _: u16,
    ctxt: *mut ble_gatt_access_ctxt,
    _: *mut c_void,
) -> i32 {
    unsafe {
        let om = (*ctxt).om;
        let len = (*om).om_len as usize;
        let data = core::slice::from_raw_parts((*om).om_data, len);

        if let Some(cb) = WRITE_CB {
            cb(data);
        }
    }
    0
}

/* ===================== GATT DEFINITIONS ===================== */

static GATT_CHR: ble_gatt_chr_def = ble_gatt_chr_def {
    uuid: &ble_uuid128_t {
        u: ble_uuid128 {
            u128: *b"\x01\xef\xcd\xab\x89\x67\x45\x23\x01\x23\x45\x67\x89\xab\xcd\xef",
        },
    } as *const _ as *const ble_uuid_t,
    access_cb: Some(gatt_access),
    flags: BLE_GATT_CHR_F_WRITE as u16,
    ..unsafe { core::mem::zeroed() }
};

static GATT_CHRS: [ble_gatt_chr_def; 2] = [
    GATT_CHR,
    ble_gatt_chr_def {
        uuid: core::ptr::null(),
        access_cb: None,
        flags: 0,
        ..unsafe { core::mem::zeroed() }
    },
];

static GATT_SVC: ble_gatt_svc_def = ble_gatt_svc_def {
    type_: BLE_GATT_SVC_TYPE_PRIMARY as u8,
    uuid: &ble_uuid128_t {
        u: ble_uuid128 {
            u128: *b"\xf0\xde\xbc\x9a\x78\x56\x34\x12\x34\x12\x56\x78\x9a\xbc\xde\xf0",
        },
    } as *const _ as *const ble_uuid_t,
    characteristics: GATT_CHRS.as_ptr(),
};

static GATT_DEFS: [ble_gatt_svc_def; 2] = [
    GATT_SVC,
    ble_gatt_svc_def {
        type_: 0,
        uuid: core::ptr::null(),
        characteristics: core::ptr::null(),
    },
];
