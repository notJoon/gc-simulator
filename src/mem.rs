#[derive(Debug, Default, PartialEq, Clone)]
pub struct Memory {
    pub status: Status,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Status {
    #[default]
    Free,
    Allocated,
    Marked,
    Used,
}

impl Memory {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn free() -> Self {
        Self {
            status: Status::Free,
        }
    }
}
