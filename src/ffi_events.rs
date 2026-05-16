//! FFI event callback interface for external integrators
//!
//! Provides C ABI interface for subscribing to core events from any language.

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::event::CoreEvent;
use crate::event_bus::EventSubscription;

/// Opaque handle to an event subscription
pub type RustdeskEventSubscription = *mut EventSubscription;

/// Callback function type for event delivery
///
/// # Parameters
/// - `event_type`: Numeric event type (see RUSTDESK_EVENT_* constants)
/// - `event_data`: Pointer to event-specific data structure
/// - `user_data`: User-provided context pointer passed during registration
pub type EventCallback = extern "C" fn(event_type: u32, event_data: *const c_void, user_data: *mut c_void);

/// Event type constants for FFI
pub const RUSTDESK_EVENT_CONNECTED: u32 = 1;
pub const RUSTDESK_EVENT_DISCONNECTED: u32 = 2;
pub const RUSTDESK_EVENT_PERMISSION_CHANGED: u32 = 3;
pub const RUSTDESK_EVENT_MSGBOX: u32 = 4;
pub const RUSTDESK_EVENT_DISPLAY_CHANGED: u32 = 5;
pub const RUSTDESK_EVENT_SWITCH_DISPLAY: u32 = 6;
pub const RUSTDESK_EVENT_QUALITY_STATUS: u32 = 7;
pub const RUSTDESK_EVENT_CURSOR_DATA: u32 = 8;
pub const RUSTDESK_EVENT_CURSOR_POSITION: u32 = 9;
pub const RUSTDESK_EVENT_JOB_PROGRESS: u32 = 10;
pub const RUSTDESK_EVENT_NEW_MESSAGE: u32 = 11;
pub const RUSTDESK_EVENT_VOICE_CALL_STARTED: u32 = 12;
pub const RUSTDESK_EVENT_VOICE_CALL_CLOSED: u32 = 13;
pub const RUSTDESK_EVENT_BLOCK_INPUT_STATE_CHANGED: u32 = 14;
pub const RUSTDESK_EVENT_TERMINAL_RESPONSE: u32 = 15;
pub const RUSTDESK_EVENT_SCREENSHOT_RESPONSE: u32 = 16;

/// Global event callback storage
static EVENT_CALLBACK: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static EVENT_USER_DATA: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Register a global event callback for all session events
///
/// # Arguments
/// - `callback`: Function pointer to receive events
/// - `user_data`: User context pointer passed to callback on each invocation
///
/// # Safety
/// The callback must remain valid for the lifetime of the subscription.
/// The callback must not panic.
/// The callback must handle all event types gracefully.
#[no_mangle]
pub unsafe extern "C" fn rustdesk_register_event_callback(
    callback: EventCallback,
    user_data: *mut c_void,
) {
    EVENT_CALLBACK.store(callback as *mut c_void, Ordering::SeqCst);
    EVENT_USER_DATA.store(user_data, Ordering::SeqCst);
}

/// Process events from a subscription, calling the registered callback for each
///
/// # Arguments
/// - `subscription`: Event subscription handle
/// - `max_events`: Maximum number of events to process (0 = process all available)
///
/// # Returns
/// Number of events processed
///
/// # Safety
/// subscription must be a valid RustdeskEventSubscription created by session_start
#[no_mangle]
pub unsafe extern "C" fn rustdesk_process_events(
    subscription: RustdeskEventSubscription,
    max_events: u32,
) -> u32 {
    if subscription.is_null() {
        return 0;
    }

    let callback_ptr = EVENT_CALLBACK.load(Ordering::SeqCst);
    if callback_ptr.is_null() {
        return 0;
    }

    let callback: EventCallback = std::mem::transmute(callback_ptr);
    let user_data = EVENT_USER_DATA.load(Ordering::SeqCst);

    let sub = &*subscription;
    let mut count = 0u32;
    let limit = if max_events == 0 { u32::MAX } else { max_events };

    while count < limit {
        if let Some(event) = sub.try_recv() {
            dispatch_event_to_callback(callback, user_data, &event);
            count += 1;
        } else {
            break;
        }
    }

    count
}

/// Dispatch a CoreEvent to the registered FFI callback
fn dispatch_event_to_callback(
    callback: EventCallback,
    user_data: *mut c_void,
    event: &CoreEvent,
) {
    let (event_type, event_data): (u32, *const c_void) = match event {
        CoreEvent::Connected { .. } => (RUSTDESK_EVENT_CONNECTED, std::ptr::null()),
        CoreEvent::Disconnected => (RUSTDESK_EVENT_DISCONNECTED, std::ptr::null()),
        CoreEvent::PermissionChanged { .. } => (RUSTDESK_EVENT_PERMISSION_CHANGED, std::ptr::null()),
        CoreEvent::MsgBox { .. } => (RUSTDESK_EVENT_MSGBOX, std::ptr::null()),
        CoreEvent::DisplayChanged { .. } => (RUSTDESK_EVENT_DISPLAY_CHANGED, std::ptr::null()),
        CoreEvent::SwitchDisplay { .. } => (RUSTDESK_EVENT_SWITCH_DISPLAY, std::ptr::null()),
        CoreEvent::QualityStatus { .. } => (RUSTDESK_EVENT_QUALITY_STATUS, std::ptr::null()),
        CoreEvent::CursorData { .. } => (RUSTDESK_EVENT_CURSOR_DATA, std::ptr::null()),
        CoreEvent::CursorPosition { .. } => (RUSTDESK_EVENT_CURSOR_POSITION, std::ptr::null()),
        CoreEvent::JobProgress { .. } => (RUSTDESK_EVENT_JOB_PROGRESS, std::ptr::null()),
        CoreEvent::NewMessage { .. } => (RUSTDESK_EVENT_NEW_MESSAGE, std::ptr::null()),
        CoreEvent::VoiceCallStarted => (RUSTDESK_EVENT_VOICE_CALL_STARTED, std::ptr::null()),
        CoreEvent::VoiceCallClosed { .. } => (RUSTDESK_EVENT_VOICE_CALL_CLOSED, std::ptr::null()),
        CoreEvent::BlockInputStateChanged { .. } => (RUSTDESK_EVENT_BLOCK_INPUT_STATE_CHANGED, std::ptr::null()),
        CoreEvent::TerminalResponse { .. } => (RUSTDESK_EVENT_TERMINAL_RESPONSE, std::ptr::null()),
        CoreEvent::ScreenshotResponse { .. } => (RUSTDESK_EVENT_SCREENSHOT_RESPONSE, std::ptr::null()),
        // Map all other events to generic handlers
        _ => (0, std::ptr::null()),
    };

    callback(event_type, event_data, user_data);
}

/// Free an event subscription
///
/// # Safety
/// subscription must be a valid RustdeskEventSubscription
/// After calling this, the subscription pointer is invalid and must not be used
#[no_mangle]
pub unsafe extern "C" fn rustdesk_free_event_subscription(subscription: RustdeskEventSubscription) {
    if !subscription.is_null() {
        drop(Box::from_raw(subscription));
    }
}