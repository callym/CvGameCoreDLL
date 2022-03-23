#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SlotStatus {
  Open,
  Computer,
  Closed,
  Taken,
  MaxSlotStatus,
}
