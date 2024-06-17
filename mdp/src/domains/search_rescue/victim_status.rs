use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum VictimStatus {
    Unknown,
    NeedAmbulance,
    Hazard,
    Handled,
}
