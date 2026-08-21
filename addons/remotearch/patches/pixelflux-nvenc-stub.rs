use smithay::backend::allocator::dmabuf::Dmabuf;
use crate::RustCaptureSettings;

/// Stub: NVENC/CUDA is not built in this image (Intel Arc / VAAPI path).
/// NvencEncoder::new always errors so lib.rs falls back to VAAPI/CPU.
pub struct NvencEncoder;

impl NvencEncoder {
    pub fn new(
        _settings: &RustCaptureSettings,
        _egl_display: *const std::ffi::c_void,
    ) -> Result<Self, String> {
        Err("NVENC disabled (no CUDA in this build)".into())
    }

    pub fn encode(
        &mut self,
        _dmabuf: &Dmabuf,
        _frame_number: u64,
        _target_qp: u32,
        _force_idr: bool,
    ) -> Result<Vec<u8>, String> {
        Err("NVENC disabled".into())
    }

    pub fn encode_raw(
        &mut self,
        _raw_data: &[u8],
        _frame_number: u64,
        _target_qp: u32,
        _force_idr: bool,
    ) -> Result<Vec<u8>, String> {
        Err("NVENC disabled".into())
    }
}
