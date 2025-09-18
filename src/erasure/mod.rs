// Erasure module - Main interface for all erasure methods
// This module coordinates between different erasure strategies

pub mod overwrite;
pub mod ata;
pub mod nvme;
pub mod crypto;

// TODO: Add main erasure coordinator struct
// TODO: Add trait for erasure methods
// TODO: Add device detection logic
// TODO: Add erasure method selection logic
// TODO: Add progress tracking interface