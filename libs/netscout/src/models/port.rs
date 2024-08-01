#[derive(Debug, Clone)]
pub struct Port {
    pub number: u16,
    pub state: PortState,
    pub service: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortState {
    Open,
    Closed,
    Filteredm
}
