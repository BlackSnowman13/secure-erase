# Secure Erase

A cross-platform storage device secure erasure tool built in Rust with a modern GUI interface. This application provides secure data wiping capabilities for HDDs, SSDs, NVMe drives, USB devices, and other storage media across Linux, Windows, and Android platforms. Available as both installable applications and bootable ISO for bare-metal operations.

**Key Capabilities:**
- **Multi-Platform Support**: Native applications for Linux, Windows, and Android, plus bootable ISO for hardware-level operations
- **Universal Device Compatibility**: Comprehensive support for all major storage device types and interfaces
- **Certified Verification**: Generates cryptographically signed certificates providing tamper-proof documentation of secure erasure completion
- **Advanced Drive Visualization**: Integrated hex viewer, partition analyzer, and real-time data mapping tools for thorough pre/post-erasure inspection

## 🗂️ Project Structure

```
secure-erase/
├── src/
│   ├── main.rs             # Application entry point and module declarations
│   ├── ui.rs               # GUI implementation using egui framework
│   ├── utils.rs            # Utility functions for device operations
│   ├── erasure/
│   │   └── ata.rs          # ATA-specific secure erase implementation
│   │   └── crypto.rs       # Cryptographic erase implementation
│   │   └── mod.rs
│   │   └── nvme.rs         # NVMe-specific secure erase implementation
│   │   └── overwrite.rs    # Overwrite erase implementation
│   └── platform/
│       ├── mod.rs          # Platform abstraction layer definitions
│       └── linux.rs        # Linux-specific storage device detection
│       └── windows.rs      # Windows-specific storage device detection
├── Cargo.toml              # Project dependencies and configuration
└── README.md               # This file
```

## 🎯 Main Features

### 🔍 **Automatic Device Detection**
- **Real-time Scanning**: Automatically detects all connected storage devices
- **Smart Classification**: Identifies device types (HDD, SSD, NVMe, USB)
- **Comprehensive Info**: Shows device model, size, vendor, serial number
- **Mount Status**: Displays whether devices are currently in use

### 🔒 **Multiple Erasure Methods**
- **ATA Secure Erase**: Hardware-level secure erasure for ATA/SATA devices
- **Enhanced Secure Erase**: More thorough erasure for compatible drives
- **Overwrite Methods**: Multiple-pass software-based wiping (planned)
- **NVMe Secure Erase**: NVMe-specific secure erase commands (planned)
- **Crypto Erase**: Self-encrypting drive cryptographic erasure (planned)

### 🛡️ **Security & Safety**
- **Permission Checks**: Verifies proper system access before operations
- **Mount Detection**: Prevents erasure of active filesystems
- **Device Validation**: Comprehensive compatibility checking
- **Frozen State Detection**: Identifies when secure erase is unavailable

### 🔍 **Drive Visualization & Analysis**
- **Interactive Partition View**: Visual representation of partition layout and disk geometry
- **Real-time Hex Viewer**: Low-level hex view of disk sectors with intelligent data highlighting
- **File System Analysis**: Comprehensive scan of active files with metadata inspection
- **Data Recovery Detection**: Identifies deleted files and recoverable data fragments
- **Visual Data Mapping**: Color-coded visualization showing file types and data distribution
- **Forensic Insights**: Displays file signatures, magic numbers, and potential hidden data

## 🎯 Usage

The application provides an intuitive GUI interface:

1. **Device Selection**: Choose from the comprehensive list of detected storage devices displayed in the left panel
2. **Device Information**: View detailed device specifications, health status, and security capabilities in the right panel
3. **Erasure Progress Monitoring**: Real-time progress visualization with estimated completion time, current operation status, and throughput metrics
4. **Digital Certification**: Generate cryptographically signed certificates upon completion, providing tamper-proof verification of the secure erasure process with timestamps and method details
5. **Pre/Post Visualization**: Use the integrated hex viewer and partition analyzer to examine device contents before erasure and verify complete data removal afterward, with optional data recovery testing to ensure thorough wiping

---

**Note**: This application requires root/administrator privileges to access storage devices directly. Always ensure you have proper backups before performing any secure erase operations.