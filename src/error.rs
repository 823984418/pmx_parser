use thiserror::Error;

#[derive(Error, Debug)]
pub enum PmxError {
    #[error("magic error")]
    MagicError,

    #[error("index error")]
    IndexError,

    #[error("global data error")]
    GlobalDataError,

    #[error("vertex count error")]
    VertexCountError,

    #[error("morph error")]
    MorphError,

    #[error("soft body form error")]
    SoftBodyFormError,

    #[error("soft body aero model error")]
    SoftBodyAeroModelError,

    #[error("joint type error")]
    JointTypeError,

    #[error("rigid form error")]
    RigidFormError,

    #[error("rigid calculate method error")]
    RigidCalcMethodError,

    #[error("display frame error")]
    DisplayFrameError,

    #[error("control panel error")]
    ControlPanelError,

    #[error("mix error")]
    MixError,

    #[error("bool error")]
    BoolError,

    #[error("toon error")]
    ToonError,

    #[error("encoding error")]
    EncodingError,

    #[error("skin error")]
    SkinError,

    #[error("global data length too long")]
    GlobalDataLengthTooLong,

    #[error("invalid encoding {0}")]
    InvalidEncoding(u8),

    #[error("invalid index size {0}")]
    InvalidIndexSize(u8),

    #[error("io error {0}")]
    Io(#[from] std::io::Error),
}
