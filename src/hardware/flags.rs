/// Enum representing the condition flags in the LC-3 VM.
///
/// Each flag corresponds to a specific bit position:
/// - `POS` (Positive) is represented by the least significant bit.
/// - `ZRO` (Zero) is represented by the second least significant bit.
/// - `NEG` (Negative) is represented by the third least significant bit.
pub enum Flag {
    /// Positive flag, indicating a positive result (P).
    POS = 1 << 0,

    /// Zero flag, indicating a zero result (Z).
    ZRO = 1 << 1,

    /// Negative flag, indicating a negative result (N).
    NEG = 1 << 2,
}
