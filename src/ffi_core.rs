use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use hbb_common::tokio::sync::mpsc;
use hbb_common::message_proto::{
    Clipboard, ClipboardFormat, CursorData, CursorPosition, DisplayInfo, FileEntry, KeyEvent,
    KeyboardMode, Message, MultiClipboards, PeerInfo, ReadEmptyDirsResponse, SwitchDisplay, TerminalResponse,
    WindowsSession,
};
use hbb_common::rendezvous_proto::ConnType;

use crate::client::QualityStatus;
use crate::ui_session_interface::{io_loop, InvokeUiSession, Session};

pub type RustdeskSession = *mut c_void;

const SESSION_STATE_CONNECTING: i32 = 0;
const SESSION_STATE_CONNECTED: i32 = 1;
const SESSION_STATE_DISCONNECTED: i32 = 2;
const SESSION_STATE_ERROR: i32 = -1;

const RUSTDESK_OK: i32 = 0;
const RUSTDESK_ERR_INVALID_ARGUMENT: i32 = -1;
const RUSTDESK_ERR_NOT_READY: i32 = -2;
const RUSTDESK_ERR_INVALID_STATE: i32 = -3;
const RUSTDESK_ERR_ENCODING: i32 = -4;

const EVENT_CONNECTED: u32 = 1;
const EVENT_DISCONNECTED: u32 = 2;
const EVENT_CLIPBOARD_UPDATED: u32 = 17;
const EVENT_READY: u32 = 18;
const EVENT_FIRST_FRAME: u32 = 19;
const EVENT_ERROR: u32 = 20;
const EVENT_AUTH_REQUIRED: u32 = 21;

#[repr(C)]
pub enum MouseEventType {
    Move = 0,
    LeftDown = 1,
    LeftUp = 2,
    RightDown = 3,
    RightUp = 4,
    Wheel = 5,
}

#[repr(C)]
pub enum CodecFormatFFI {
    VP9 = 0,
    VP8 = 1,
    AV1 = 2,
    H264 = 3,
    H265 = 4,
    RawRgba = 255,
}

#[repr(C)]
pub struct VideoFrameData {
    pub data: *mut u8,
    pub data_len: usize,
    pub width: u32,
    pub height: u32,
    pub format: CodecFormatFFI,
    pub timestamp: i64,
    pub key_frame: bool,
}

pub type EventCallback = extern "C" fn(event_type: u32, event_data: *const c_void, user_data: *mut c_void);

static EVENT_CALLBACK: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static EVENT_USER_DATA: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

#[derive(Default)]
struct FrameBuffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
    ready: bool,
    first_frame_emitted: bool,
}

#[derive(Default)]
struct CoreSessionShared {
    state: AtomicI32,
    last_error: Mutex<String>,
    frame: Mutex<FrameBuffer>,
    ready: AtomicBool,
    password: Mutex<String>,
    sender: Arc<RwLock<Option<mpsc::UnboundedSender<crate::client::Data>>>>,
    auth_sent: AtomicBool,
}

#[derive(Clone, Default)]
struct CoreHandler {
    shared: Arc<CoreSessionShared>,
}

struct CoreSessionHandle {
    session: Session<CoreHandler>,
}

fn emit_event(event_type: u32) {
    let cb_ptr = EVENT_CALLBACK.load(Ordering::SeqCst);
    if cb_ptr.is_null() {
        return;
    }
    let user_data = EVENT_USER_DATA.load(Ordering::SeqCst);
    let callback: EventCallback = unsafe { std::mem::transmute(cb_ptr) };
    callback(event_type, ptr::null(), user_data);
}

impl CoreSessionHandle {
    fn new(peer_id: String, password: String) -> Self {
        let sender = Arc::new(RwLock::new(None));
        let thread = Arc::new(Mutex::new(None));
        let handler = CoreHandler {
            shared: Arc::new(CoreSessionShared {
                state: AtomicI32::new(SESSION_STATE_CONNECTING),
                last_error: Mutex::new(String::new()),
                frame: Mutex::new(FrameBuffer::default()),
                ready: AtomicBool::new(false),
                password: Mutex::new(password.clone()),
                sender: sender.clone(),
                auth_sent: AtomicBool::new(false),
            }),
        };
        let session: Session<CoreHandler> = Session {
            password,
            args: vec![],
            sender: sender.clone(),
            thread: thread.clone(),
            ui_handler: handler.clone(),
            server_keyboard_enabled: Arc::new(std::sync::RwLock::new(true)),
            server_file_transfer_enabled: Arc::new(std::sync::RwLock::new(true)),
            server_clipboard_enabled: Arc::new(std::sync::RwLock::new(true)),
            reconnect_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            ..Default::default()
        };
        session.lc.write().unwrap().initialize(
            peer_id,
            ConnType::DEFAULT_CONN,
            None,
            false,
            None,
            None,
            None,
        );
        Self { session }
    }

    fn start(&self) {
        let round = self.session.connection_round_state.lock().unwrap().new_round();
        let cloned = self.session.clone();
        let mut lock = self.session.thread.lock().unwrap();
        *lock = Some(std::thread::spawn(move || {
            io_loop(cloned, round);
        }));
    }
}

impl InvokeUiSession for CoreHandler {
    fn set_cursor_data(&self, _cd: CursorData) {}
    fn set_cursor_id(&self, _id: String) {}
    fn set_cursor_position(&self, _cp: CursorPosition) {}
    fn set_display(&self, _x: i32, _y: i32, _w: i32, _h: i32, _cursor_embedded: bool, _scale: f64) {}
    fn switch_display(&self, _display: &SwitchDisplay) {}
    fn set_peer_info(&self, _peer_info: &PeerInfo) {
        if !self.shared.ready.swap(true, Ordering::SeqCst) {
            emit_event(EVENT_READY);
        }
    }
    fn set_displays(&self, _displays: &Vec<DisplayInfo>) {}
    fn set_platform_additions(&self, _data: &str) {}
    fn on_connected(&self, _conn_type: ConnType) {
        self.shared.state.store(SESSION_STATE_CONNECTED, Ordering::SeqCst);
        self.shared.auth_sent.store(false, Ordering::SeqCst);
        emit_event(EVENT_CONNECTED);
    }
    fn update_privacy_mode(&self) {}
    fn set_permission(&self, _name: &str, _value: bool) {}
    fn close_success(&self) {
        self.shared
            .state
            .store(SESSION_STATE_DISCONNECTED, Ordering::SeqCst);
        emit_event(EVENT_DISCONNECTED);
    }
    fn update_quality_status(&self, _qs: QualityStatus) {}
    fn set_connection_type(&self, _is_secured: bool, _direct: bool, _stream_type: &str) {}
    fn set_fingerprint(&self, _fingerprint: String) {}
    fn job_error(&self, _id: i32, _err: String, _file_num: i32) {}
    fn job_done(&self, _id: i32, _file_num: i32) {}
    fn clear_all_jobs(&self) {}
    fn new_message(&self, _msg: String) {}
    fn update_transfer_list(&self) {}
    fn load_last_job(&self, _cnt: i32, _job_json: &str, _auto_start: bool) {}
    fn update_folder_files(&self, _id: i32, _entries: &Vec<FileEntry>, _path: String, _is_local: bool, _only_count: bool) {}
    fn confirm_delete_files(&self, _id: i32, _i: i32, _name: String) {}
    fn override_file_confirm(&self, _id: i32, _file_num: i32, _to: String, _is_upload: bool, _is_identical: bool) {}
    fn update_block_input_state(&self, _on: bool) {}
    fn job_progress(&self, _id: i32, _file_num: i32, _speed: f64, _finished_size: f64) {}
    fn adapt_size(&self) {}
    fn on_rgba(&self, _display: usize, rgba: &mut scrap::ImageRgb) {
        let mut frame = self.shared.frame.lock().unwrap();
        frame.data.clear();
        frame.data.extend_from_slice(&rgba.raw);
        frame.width = rgba.w as u32;
        frame.height = rgba.h as u32;
        frame.ready = true;
        if !frame.first_frame_emitted {
            frame.first_frame_emitted = true;
            emit_event(EVENT_FIRST_FRAME);
        }
    }
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, _link: &str, _retry: bool) {
        if msgtype == "input-password" || msgtype == "re-input-password" {
            emit_event(EVENT_AUTH_REQUIRED);
            if !self.shared.auth_sent.swap(true, Ordering::SeqCst) {
                let password = self.shared.password.lock().unwrap().clone();
                if let Some(sender) = self.shared.sender.read().unwrap().as_ref() {
                    sender
                        .send(crate::client::Data::Login((
                            "".to_owned(),
                            "".to_owned(),
                            password,
                            true,
                        )))
                        .ok();
                }
            }
        }
        if msgtype.contains("error") {
            self.shared.state.store(SESSION_STATE_ERROR, Ordering::SeqCst);
            let mut err = self.shared.last_error.lock().unwrap();
            *err = if title.is_empty() {
                text.to_owned()
            } else {
                format!("{}: {}", title, text)
            };
            emit_event(EVENT_ERROR);
        }
    }
    fn cancel_msgbox(&self, _tag: &str) {}
    fn switch_back(&self, _id: &str) {}
    fn portable_service_running(&self, _running: bool) {}
    fn on_voice_call_started(&self) {}
    fn on_voice_call_closed(&self, _reason: &str) {}
    fn on_voice_call_waiting(&self) {}
    fn on_voice_call_incoming(&self) {}
    fn get_rgba(&self, _display: usize) -> *const u8 { ptr::null() }
    fn next_rgba(&self, _display: usize) {}
    fn set_multiple_windows_session(&self, _sessions: Vec<WindowsSession>) {}
    fn set_current_display(&self, _disp_idx: i32) {}
    fn update_record_status(&self, _start: bool) {}
    fn printer_request(&self, _id: i32, _path: String) {}
    fn handle_screenshot_resp(&self, _sid: String, _msg: String) {}
    fn handle_terminal_response(&self, _response: TerminalResponse) {}
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_register_event_callback(callback: Option<EventCallback>, user_data: *mut c_void) {
    let cb_ptr = callback.map(|cb| cb as *mut c_void).unwrap_or(ptr::null_mut());
    EVENT_CALLBACK.store(cb_ptr, Ordering::SeqCst);
    EVENT_USER_DATA.store(user_data, Ordering::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_connect(peer_id: *const c_char, password: *const c_char, _options: *const c_void) -> RustdeskSession {
    if peer_id.is_null() {
        return ptr::null_mut();
    }
    let peer_id = match CStr::from_ptr(peer_id).to_str() {
        Ok(s) if !s.is_empty() => s.to_owned(),
        _ => return ptr::null_mut(),
    };
    let password = if password.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(password).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => String::new(),
        }
    };
    let handle = Box::new(CoreSessionHandle::new(peer_id, password));
    handle.start();
    Box::into_raw(handle) as RustdeskSession
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_disconnect(session: RustdeskSession) -> i32 {
    if session.is_null() {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let handle = Box::from_raw(session as *mut CoreSessionHandle);
    handle
        .session
        .ui_handler
        .shared
        .state
        .store(SESSION_STATE_DISCONNECTED, Ordering::SeqCst);
    handle.session.close();
    emit_event(EVENT_DISCONNECTED);
    RUSTDESK_OK
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_session_get_state(session: RustdeskSession, error_msg: *mut c_char, error_msg_len: i32) -> i32 {
    if session.is_null() {
        return SESSION_STATE_ERROR;
    }
    let handle = &*(session as *mut CoreSessionHandle);
    let state = handle.session.ui_handler.shared.state.load(Ordering::SeqCst);
    if !error_msg.is_null() && error_msg_len > 0 && state == SESSION_STATE_ERROR {
        let err = handle.session.ui_handler.shared.last_error.lock().unwrap().clone();
        let bytes = err.as_bytes();
        let max = (error_msg_len as usize).saturating_sub(1);
        let len = bytes.len().min(max);
        ptr::copy_nonoverlapping(bytes.as_ptr(), error_msg as *mut u8, len);
        *error_msg.add(len) = 0;
    }
    state
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_get_frame(session: RustdeskSession) -> *mut VideoFrameData {
    if session.is_null() {
        return ptr::null_mut();
    }
    let handle = &*(session as *mut CoreSessionHandle);
    let frame = handle.session.ui_handler.shared.frame.lock().unwrap();
    if !frame.ready || frame.data.is_empty() {
        return ptr::null_mut();
    }
    let mut data = frame.data.clone();
    let ptr_data = data.as_mut_ptr();
    let len = data.len();
    std::mem::forget(data);
    Box::into_raw(Box::new(VideoFrameData {
        data: ptr_data,
        data_len: len,
        width: frame.width,
        height: frame.height,
        format: CodecFormatFFI::RawRgba,
        timestamp: 0,
        key_frame: false,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_free_frame(frame: *mut VideoFrameData) {
    if frame.is_null() {
        return;
    }
    let frame_box = Box::from_raw(frame);
    if !frame_box.data.is_null() && frame_box.data_len > 0 {
        let _ = Vec::from_raw_parts(frame_box.data, frame_box.data_len, frame_box.data_len);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_get_queue_size(session: RustdeskSession) -> i32 {
    if session.is_null() {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let handle = &*(session as *mut CoreSessionHandle);
    let frame = handle.session.ui_handler.shared.frame.lock().unwrap();
    if frame.ready && !frame.data.is_empty() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_send_mouse_event(
    session: RustdeskSession,
    event_type: MouseEventType,
    x: i32,
    y: i32,
    buttons: i32,
) -> i32 {
    if session.is_null() {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let handle = &*(session as *mut CoreSessionHandle);
    if handle.session.sender.read().unwrap().is_none() {
        return RUSTDESK_ERR_NOT_READY;
    }
    let event_mask = match event_type {
        MouseEventType::Move => 0,
        MouseEventType::LeftDown => crate::input::MOUSE_TYPE_DOWN | (crate::input::MOUSE_BUTTON_LEFT << 3),
        MouseEventType::LeftUp => crate::input::MOUSE_TYPE_UP | (crate::input::MOUSE_BUTTON_LEFT << 3),
        MouseEventType::RightDown => crate::input::MOUSE_TYPE_DOWN | (crate::input::MOUSE_BUTTON_RIGHT << 3),
        MouseEventType::RightUp => crate::input::MOUSE_TYPE_UP | (crate::input::MOUSE_BUTTON_RIGHT << 3),
        MouseEventType::Wheel => crate::input::MOUSE_TYPE_WHEEL,
    } | buttons;
    handle.session.send_mouse(event_mask, x, y, false, false, false, false);
    RUSTDESK_OK
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_send_key_event(session: RustdeskSession, keycode: i32, down: bool, _modifiers: i32) -> i32 {
    if session.is_null() {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let handle = &*(session as *mut CoreSessionHandle);
    if handle.session.sender.read().unwrap().is_none() {
        return RUSTDESK_ERR_NOT_READY;
    }
    if !(0..=0x10FFFF).contains(&keycode) {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let mut evt = KeyEvent::new();
    evt.down = down;
    evt.press = !down;
    evt.mode = KeyboardMode::Legacy.into();
    evt.set_chr(keycode as u32);
    handle.session.send_key_event(&evt);
    RUSTDESK_OK
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_send_clipboard(session: RustdeskSession, text: *const c_char, _len: i32) -> i32 {
    if session.is_null() || text.is_null() {
        return RUSTDESK_ERR_INVALID_ARGUMENT;
    }
    let handle = &*(session as *mut CoreSessionHandle);
    if handle.session.sender.read().unwrap().is_none() {
        return RUSTDESK_ERR_NOT_READY;
    }
    let text = match CStr::from_ptr(text).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return RUSTDESK_ERR_ENCODING,
    };
    let mut msg = Message::new();
    let clipboard = Clipboard {
        compress: false,
        content: text.into_bytes().into(),
        format: ClipboardFormat::Text.into(),
        ..Default::default()
    };
    msg.set_multi_clipboards(MultiClipboards {
        clipboards: vec![clipboard],
        ..Default::default()
    });
    if let Some(sender) = handle.session.sender.read().unwrap().as_ref() {
        sender.send(crate::client::Data::Message(msg)).ok();
    } else {
        return -1;
    }
    emit_event(EVENT_CLIPBOARD_UPDATED);
    RUSTDESK_OK
}

#[no_mangle]
pub unsafe extern "C" fn rustdesk_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}
