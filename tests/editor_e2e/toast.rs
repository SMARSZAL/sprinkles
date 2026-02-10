use std::time::Duration;

use sprinkles_editor::ui::components::toasts::ToastEvent;
use sprinkles_editor::ui::widgets::toast::{ToastVariant, DEFAULT_TOAST_DURATION};

#[test]
fn test_toast_info_construction() {
    let toast = ToastEvent::info("Hello");
    assert!(matches!(toast.variant, ToastVariant::Info));
    assert_eq!(toast.content, "Hello");
    assert_eq!(toast.duration, DEFAULT_TOAST_DURATION);
}

#[test]
fn test_toast_error_construction() {
    let toast = ToastEvent::error("Failed");
    assert!(matches!(toast.variant, ToastVariant::Error));
    assert_eq!(toast.content, "Failed");
}

#[test]
fn test_toast_custom_duration() {
    let toast = ToastEvent::info("Saving...").with_duration(Duration::from_secs(10));
    assert_eq!(toast.duration, Duration::from_secs(10));
    assert_eq!(toast.content, "Saving...");
}
