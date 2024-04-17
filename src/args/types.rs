#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VideoCodec {
    H264, AV1, VP9
}
impl Default for VideoCodec {
    fn default() -> Self {
        Self::H264
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VideoQuality {
    pub quality: u16
}
impl Default for VideoQuality {
    fn default() -> Self {
        Self { quality: 1080 }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AudioFormat {
    BEST, MP3, OGG, WAV, OPUS
}
impl Default for AudioFormat {
    fn default() -> Self {
        Self::MP3
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Help {
    Get, List, Bulk, Help, Examples
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Method {
    Get, List, Bulk, Help, Version, CobaltVersion
}

#[derive(Debug, PartialEq, Eq)]
enum ParseErrType {
    InvalidArg,
    Incomplete
}
#[derive(Debug)]
pub struct ParseError {
    err_type: ParseErrType,
    message: String
}
impl ParseError {
    pub fn throw_incomplete(message: &str) -> Self {
        Self {
            err_type: ParseErrType::Incomplete,
            message: message.to_string()
        }
    }
    pub fn throw_invalid(message: &str) -> Self {
        Self {
            err_type: ParseErrType::InvalidArg,
            message: message.to_string()
        }
    }
    pub fn print(&self) -> String {
        format!("{:?}: {}", self.err_type , &self.message)
    }
}
