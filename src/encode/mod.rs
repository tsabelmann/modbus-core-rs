pub mod response;
pub mod request;

use crate::codes::FunctionKind;

// ── FrameEncoder ──

pub trait FrameEncoder {
    /// Mutable access to the payload area (after function code).
    fn payload_mut(&mut self) -> &mut [u8];

    /// Maximum payload capacity.
    fn payload_capacity(&self) -> usize;

    /// Finalize the frame with function code and payload length.
    fn finalize(&mut self, code: FunctionKind, payload_len: usize);
}
