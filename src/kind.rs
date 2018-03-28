#[derive(Debug, Clone)]
pub enum CunninghamKind {
    FIRST = 1,
    SECOND = 2,
    BITWIN = 3
}

pub enum CunninghamError {
    InvalidKind(String)
}

impl CunninghamKind {
    pub fn from_u32(value: u32) -> Result<CunninghamKind, CunninghamError> {
        match value {
            1 => Ok(CunninghamKind::FIRST),
            2 => Ok(CunninghamKind::SECOND),
            3 => Ok(CunninghamKind::BITWIN),
            _ => Err(CunninghamError::InvalidKind(format!("Invalid kind selected. Cannot be {}", value)))
        }
    }
}
