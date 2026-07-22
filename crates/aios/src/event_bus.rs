//! # AIOS Event Bus
//!
//! A lightweight mpsc channel that AIOS operations (module loader, workflow
//! runner, brain scanner, constitution loader) write to so that the TUI can
//! display live process status without polling.
//!
//! ## Usage
//!
//! ```ignore
//! let (tx, rx) = aios_channel();
//!
//! // In AIOS operations:
//! tx.send_started("module-scan", "scanning registry");
//! tx.send_progress("module-scan", "loaded AIOS-BRAIN-000006");
//! tx.send_done("module-scan");
//!
//! // In TUI event loop (drain every render tick):
//! while let Ok(ev) = rx.try_recv() {
//!     app.apply_aios_event(ev);
//! }
//! ```

use std::sync::mpsc;

// ── Event types ──────────────────────────────────────────────────────────────

/// An event emitted by an AIOS background operation.
#[derive(Debug, Clone)]
pub enum AiosEvent {
    /// A new process has started. `label` is the stable key used to match
    /// subsequent `Progress`, `Done`, and `Failed` events.
    ProcessStarted {
        /// Short stable key, e.g. `"module-scan"`, `"workflow:feature-delivery"`.
        label: String,
        /// Human-readable initial phase / detail.
        detail: String,
    },

    /// An already-started process has a new status line.
    ProcessProgress { label: String, detail: String },

    /// A process completed successfully.
    ProcessDone { label: String },

    /// A process failed with an error message.
    ProcessFailed { label: String, error: String },
}

// ── Sender ───────────────────────────────────────────────────────────────────

/// Cloneable sender end of the AIOS event bus.
///
/// Can be passed into any AIOS operation (including across threads/tasks).
/// All `send_*` methods are fire-and-forget — they silently drop errors so
/// operations never block or fail just because the TUI receiver has been
/// dropped.
#[derive(Clone, Debug)]
pub struct AiosEventSender(mpsc::Sender<AiosEvent>);

impl AiosEventSender {
    /// Announce that a new process is starting.
    pub fn send_started(&self, label: impl Into<String>, detail: impl Into<String>) {
        let _ = self.0.send(AiosEvent::ProcessStarted {
            label: label.into(),
            detail: detail.into(),
        });
    }

    /// Update the current phase / status of an in-flight process.
    pub fn send_progress(&self, label: impl Into<String>, detail: impl Into<String>) {
        let _ = self.0.send(AiosEvent::ProcessProgress {
            label: label.into(),
            detail: detail.into(),
        });
    }

    /// Mark a process as successfully completed.
    pub fn send_done(&self, label: impl Into<String>) {
        let _ = self.0.send(AiosEvent::ProcessDone {
            label: label.into(),
        });
    }

    /// Mark a process as failed.
    pub fn send_failed(&self, label: impl Into<String>, error: impl Into<String>) {
        let _ = self.0.send(AiosEvent::ProcessFailed {
            label: label.into(),
            error: error.into(),
        });
    }
}

// ── Receiver ─────────────────────────────────────────────────────────────────

/// Non-blocking receiver end of the AIOS event bus.
///
/// Call `try_drain()` once per TUI render tick to collect all pending events.
pub struct AiosEventReceiver(mpsc::Receiver<AiosEvent>);

impl AiosEventReceiver {
    /// Drain all pending events without blocking.
    /// Returns a `Vec` so callers can process them in a simple `for` loop.
    pub fn try_drain(&self) -> Vec<AiosEvent> {
        let mut out = Vec::new();
        while let Ok(ev) = self.0.try_recv() {
            out.push(ev);
        }
        out
    }
}

// ── Constructor ──────────────────────────────────────────────────────────────

/// Create a new AIOS event bus pair `(sender, receiver)`.
pub fn aios_channel() -> (AiosEventSender, AiosEventReceiver) {
    let (tx, rx) = mpsc::channel();
    (AiosEventSender(tx), AiosEventReceiver(rx))
}
