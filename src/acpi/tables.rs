use std::collections::HashMap;

/// ACPI table header - common to all ACPI tables
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],    // Table signature
    pub length: u32,           // Length of table
    pub revision: u8,          // Revision
    pub checksum: u8,          // Checksum
    pub oem_id: [u8; 6],       // OEM ID
    pub oem_table_id: [u8; 8], // OEM table ID
    pub oem_revision: u32,     // OEM revision
    pub creator_id: u32,       // Creator ID
    pub creator_revision: u32, // Creator revision
}

impl AcpiTableHeader {
    pub fn new(signature: &[u8; 4], length: u32, revision: u8) -> Self {
        Self {
            signature: *signature,
            length,
            revision,
            checksum: 0, // Will be calculated later
            oem_id: *b"SOFTEM",
            oem_table_id: *b"SOFTEMU ",
            oem_revision: 1,
            creator_id: u32::from_le_bytes(*b"RUST"),
            creator_revision: 1,
        }
    }

    pub fn calculate_checksum(&mut self, data: &[u8]) {
        let mut sum: u8 = 0;
        for byte in data {
            sum = sum.wrapping_add(*byte);
        }
        self.checksum = (!sum).wrapping_add(1);
    }
}

/// RSDP (Root System Description Pointer) - ACPI 1.0
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpV1 {
    pub signature: [u8; 8],    // "RSD PTR "
    pub checksum: u8,          // Checksum
    pub oem_id: [u8; 6],       // OEM ID
    pub revision: u8,          // Revision
    pub rsdt_address: u32,     // RSDT address
}

/// RSDP (Root System Description Pointer) - ACPI 2.0+
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpV2 {
    pub v1: RsdpV1,            // ACPI 1.0 fields
    pub length: u32,           // Length of table
    pub xsdt_address: u64,     // XSDT address
    pub extended_checksum: u8, // Extended checksum
    pub reserved: [u8; 3],     // Reserved
}

impl RsdpV1 {
    pub fn new(rsdt_address: u32) -> Self {
        Self {
            signature: *crate::acpi::RSDP_SIGNATURE,
            checksum: 0, // Will be calculated
            oem_id: *b"SOFTEM",
            revision: crate::acpi::ACPI_VERSION_1_0,
            rsdt_address,
        }
    }

    pub fn calculate_checksum(&mut self) {
        let data = unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>()
            )
        };
        let mut sum: u8 = 0;
        for byte in data {
            sum = sum.wrapping_add(*byte);
        }
        self.checksum = (!sum).wrapping_add(1);
    }
}

impl RsdpV2 {
    pub fn new(rsdt_address: u32, xsdt_address: u64) -> Self {
        Self {
            v1: RsdpV1::new(rsdt_address),
            length: std::mem::size_of::<Self>() as u32,
            xsdt_address,
            extended_checksum: 0, // Will be calculated
            reserved: [0; 3],
        }
    }

    pub fn calculate_checksum(&mut self) {
        // Calculate v1 checksum
        self.v1.calculate_checksum();
        
        // Calculate extended checksum
        let data = unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>()
            )
        };
        let mut sum: u8 = 0;
        for byte in data {
            sum = sum.wrapping_add(*byte);
        }
        self.extended_checksum = (!sum).wrapping_add(1);
    }
}

/// RSDT (Root System Description Table)
pub struct Rsdt {
    pub header: AcpiTableHeader,
    pub entries: Vec<u32>, // Array of table addresses
}

impl Rsdt {
    pub fn new(entries: Vec<u32>) -> Self {
        let length = std::mem::size_of::<AcpiTableHeader>() as u32 + 
                    (entries.len() * 4) as u32;
        let header = AcpiTableHeader::new(crate::acpi::RSDT_SIGNATURE, length, 1);
        
        let mut rsdt = Self { header, entries };
        rsdt.calculate_checksum();
        rsdt
    }

    pub fn calculate_checksum(&mut self) {
        let data = self.to_bytes();
        self.header.calculate_checksum(&data);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add header
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const AcpiTableHeader as *const u8,
                std::mem::size_of::<AcpiTableHeader>()
            )
        };
        bytes.extend_from_slice(header_bytes);
        
        // Add entries
        let entries = self.entries.clone();
        for entry in entries {
            bytes.extend_from_slice(&entry.to_le_bytes());
        }
        
        bytes
    }
}

/// XSDT (Extended System Description Table)
pub struct Xsdt {
    pub header: AcpiTableHeader,
    pub entries: Vec<u64>, // Array of table addresses
}

impl Xsdt {
    pub fn new(entries: Vec<u64>) -> Self {
        let length = std::mem::size_of::<AcpiTableHeader>() as u32 + 
                    (entries.len() * 8) as u32;
        let header = AcpiTableHeader::new(crate::acpi::XSDT_SIGNATURE, length, 1);
        
        let mut xsdt = Self { header, entries };
        xsdt.calculate_checksum();
        xsdt
    }

    pub fn calculate_checksum(&mut self) {
        let data = self.to_bytes();
        self.header.calculate_checksum(&data);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add header
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const AcpiTableHeader as *const u8,
                std::mem::size_of::<AcpiTableHeader>()
            )
        };
        bytes.extend_from_slice(header_bytes);
        
        // Add entries
        let entries = self.entries.clone();
        for entry in entries {
            bytes.extend_from_slice(&entry.to_le_bytes());
        }
        
        bytes
    }
}

/// FADT (Fixed ACPI Description Table)
#[repr(C, packed)]
#[derive(Debug)]
pub struct Fadt {
    pub header: AcpiTableHeader,
    pub firmware_ctrl: u32,        // Firmware control
    pub dsdt: u32,                 // DSDT address
    pub reserved: u8,              // Reserved
    pub preferred_pm_profile: u8,  // Preferred power management profile
    pub sci_interrupt: u16,        // SCI interrupt
    pub smi_command_port: u32,     // SMI command port
    pub acpi_enable: u8,           // ACPI enable
    pub acpi_disable: u8,          // ACPI disable
    pub s4bios_req: u8,            // S4BIOS request
    pub pstate_control: u8,        // P-state control
    pub pm1a_event_block: u32,     // PM1a event block
    pub pm1b_event_block: u32,     // PM1b event block
    pub pm1a_control_block: u32,   // PM1a control block
    pub pm1b_control_block: u32,   // PM1b control block
    pub pm2_control_block: u32,    // PM2 control block
    pub pm_timer_block: u32,       // PM timer block
    pub gpe0_block: u32,           // GPE0 block
    pub gpe1_block: u32,           // GPE1 block
    pub pm1_event_length: u8,      // PM1 event length
    pub pm1_control_length: u8,    // PM1 control length
    pub pm2_control_length: u8,    // PM2 control length
    pub pm_timer_length: u8,       // PM timer length
    pub gpe0_block_length: u8,     // GPE0 block length
    pub gpe1_block_length: u8,     // GPE1 block length
    pub gpe1_base: u8,             // GPE1 base
    pub cstate_control: u8,        // C-state control
    pub worst_c2_latency: u16,     // Worst C2 latency
    pub worst_c3_latency: u16,     // Worst C3 latency
    pub flush_size: u16,           // Flush size
    pub flush_stride: u16,         // Flush stride
    pub duty_offset: u8,           // Duty offset
    pub duty_width: u8,            // Duty width
    pub day_alarm: u8,             // Day alarm
    pub month_alarm: u8,           // Month alarm
    pub century: u8,               // Century
    pub iapc_boot_arch: u16,       // IAPC boot architecture
    pub reserved2: u8,             // Reserved
    pub flags: u32,                // Flags
    pub reset_reg: [u8; 12],       // Reset register
    pub reset_value: u8,           // Reset value
    pub arm_boot_arch: u16,        // ARM boot architecture
    pub fadt_minor_version: u8,    // FADT minor version
    pub x_firmware_ctrl: u64,      // Extended firmware control
    pub x_dsdt: u64,               // Extended DSDT
    pub x_pm1a_event_block: [u8; 12], // Extended PM1a event block
    pub x_pm1b_event_block: [u8; 12], // Extended PM1b event block
    pub x_pm1a_control_block: [u8; 12], // Extended PM1a control block
    pub x_pm1b_control_block: [u8; 12], // Extended PM1b control block
    pub x_pm2_control_block: [u8; 12],  // Extended PM2 control block
    pub x_pm_timer_block: [u8; 12],     // Extended PM timer block
    pub x_gpe0_block: [u8; 12],         // Extended GPE0 block
    pub x_gpe1_block: [u8; 12],         // Extended GPE1 block
    pub sleep_control_reg: [u8; 12],    // Sleep control register
    pub sleep_status_reg: [u8; 12],     // Sleep status register
    pub hypervisor_vendor_id: u64,      // Hypervisor vendor ID
}

impl Fadt {
    pub fn new(dsdt_address: u64) -> Self {
        let length = std::mem::size_of::<Self>() as u32;
        let header = AcpiTableHeader::new(crate::acpi::FADT_SIGNATURE, length, 6);
        
        let mut fadt = Self {
            header,
            firmware_ctrl: 0,
            dsdt: dsdt_address as u32,
            reserved: 0,
            preferred_pm_profile: 0, // Desktop
            sci_interrupt: 9,        // IRQ 9
            smi_command_port: 0xB2,  // SMI command port
            acpi_enable: 0xA0,       // ACPI enable
            acpi_disable: 0xA1,      // ACPI disable
            s4bios_req: 0,
            pstate_control: 0,
            pm1a_event_block: 0x1000,    // PM1a event block
            pm1b_event_block: 0,
            pm1a_control_block: 0x1004,  // PM1a control block
            pm1b_control_block: 0,
            pm2_control_block: 0,
            pm_timer_block: 0x1008,      // PM timer block
            gpe0_block: 0x1020,          // GPE0 block
            gpe1_block: 0,
            pm1_event_length: 2,
            pm1_control_length: 2,
            pm2_control_length: 0,
            pm_timer_length: 4,
            gpe0_block_length: 8,
            gpe1_block_length: 0,
            gpe1_base: 0,
            cstate_control: 0,
            worst_c2_latency: 0,
            worst_c3_latency: 0,
            flush_size: 0,
            flush_stride: 0,
            duty_offset: 0,
            duty_width: 0,
            day_alarm: 0,
            month_alarm: 0,
            century: 0,
            iapc_boot_arch: 0,
            reserved2: 0,
            flags: 0x00000000, // ACPI enabled, legacy devices
            reset_reg: [0; 12],
            reset_value: 0,
            arm_boot_arch: 0,
            fadt_minor_version: 5,
            x_firmware_ctrl: 0,
            x_dsdt: dsdt_address,
            x_pm1a_event_block: [0; 12],
            x_pm1b_event_block: [0; 12],
            x_pm1a_control_block: [0; 12],
            x_pm1b_control_block: [0; 12],
            x_pm2_control_block: [0; 12],
            x_pm_timer_block: [0; 12],
            x_gpe0_block: [0; 12],
            x_gpe1_block: [0; 12],
            sleep_control_reg: [0; 12],
            sleep_status_reg: [0; 12],
            hypervisor_vendor_id: 0,
        };
        
        fadt.calculate_checksum();
        fadt
    }

    pub fn calculate_checksum(&mut self) {
        let data = self.to_bytes();
        self.header.calculate_checksum(&data);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>()
            ).to_vec()
        }
    }
}

/// MADT (Multiple APIC Description Table)
pub struct Madt {
    pub header: AcpiTableHeader,
    pub local_apic_address: u32,  // Local APIC address
    pub flags: u32,               // Flags
    pub entries: Vec<u8>,         // APIC entries
}

impl Madt {
    pub fn new(local_apic_address: u32) -> Self {
        let length = std::mem::size_of::<AcpiTableHeader>() as u32 + 8;
        let header = AcpiTableHeader::new(crate::acpi::MADT_SIGNATURE, length, 4);
        
        let mut madt = Self {
            header,
            local_apic_address,
            flags: 0, // PC-AT compatible
            entries: Vec::new(),
        };
        
        madt.calculate_checksum();
        madt
    }

    pub fn add_local_apic(&mut self, processor_id: u8, apic_id: u8, flags: u32) {
        let entry = LocalApicEntry {
            entry_type: 0, // Processor Local APIC
            length: 8,
            processor_id,
            apic_id,
            flags,
        };
        
        let entry_bytes = unsafe {
            std::slice::from_raw_parts(
                &entry as *const LocalApicEntry as *const u8,
                std::mem::size_of::<LocalApicEntry>()
            )
        };
        
        self.entries.extend_from_slice(entry_bytes);
        self.header.length += 8;
        self.calculate_checksum();
    }

    pub fn calculate_checksum(&mut self) {
        let data = self.to_bytes();
        self.header.calculate_checksum(&data);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add header
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const AcpiTableHeader as *const u8,
                std::mem::size_of::<AcpiTableHeader>()
            )
        };
        bytes.extend_from_slice(header_bytes);
        
        // Add local APIC address and flags
        bytes.extend_from_slice(&self.local_apic_address.to_le_bytes());
        bytes.extend_from_slice(&self.flags.to_le_bytes());
        
        // Add entries
        let entries = self.entries.clone();
        bytes.extend_from_slice(&entries);
        
        bytes
    }
}

/// Local APIC entry in MADT
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicEntry {
    pub entry_type: u8,    // Entry type (0 = Processor Local APIC)
    pub length: u8,        // Entry length
    pub processor_id: u8,  // Processor ID
    pub apic_id: u8,       // APIC ID
    pub flags: u32,        // Flags
}

/// MCFG (Memory Mapped Configuration Space Access Table)
pub struct Mcfg {
    pub header: AcpiTableHeader,
    pub reserved: u64,     // Reserved
    pub entries: Vec<McfgEntry>, // Configuration space entries
}

impl Mcfg {
    pub fn new() -> Self {
        let length = std::mem::size_of::<AcpiTableHeader>() as u32 + 8;
        let header = AcpiTableHeader::new(crate::acpi::MCFG_SIGNATURE, length, 1);
        
        let mut mcfg = Self {
            header,
            reserved: 0,
            entries: Vec::new(),
        };
        
        mcfg.calculate_checksum();
        mcfg
    }

    pub fn add_entry(&mut self, base_address: u64, segment: u16, start_bus: u8, end_bus: u8) {
        let entry = McfgEntry {
            base_address,
            segment,
            start_bus,
            end_bus,
            reserved: 0,
        };
        
        self.entries.push(entry);
        self.header.length += 16;
        self.calculate_checksum();
    }

    pub fn calculate_checksum(&mut self) {
        let data = self.to_bytes();
        self.header.calculate_checksum(&data);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add header
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const AcpiTableHeader as *const u8,
                std::mem::size_of::<AcpiTableHeader>()
            )
        };
        bytes.extend_from_slice(header_bytes);
        
        // Add reserved field
        bytes.extend_from_slice(&self.reserved.to_le_bytes());
        
        // Add entries
        let entries = self.entries.clone();
        for entry in entries {
            let entry_bytes = unsafe {
                std::slice::from_raw_parts(
                    &entry as *const McfgEntry as *const u8,
                    std::mem::size_of::<McfgEntry>()
                )
            };
            bytes.extend_from_slice(entry_bytes);
        }
        
        bytes
    }
}

/// MCFG entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct McfgEntry {
    pub base_address: u64, // Base address
    pub segment: u16,      // PCI segment
    pub start_bus: u8,     // Start bus
    pub end_bus: u8,       // End bus
    pub reserved: u32,     // Reserved
}

/// ACPI table manager
pub struct AcpiTableManager {
    tables: HashMap<String, Vec<u8>>,
    next_address: u64,
}

impl AcpiTableManager {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            next_address: crate::acpi::ACPI_TABLES_BASE,
        }
    }

    pub fn add_table(&mut self, signature: &str, data: Vec<u8>) -> u64 {
        let address = self.next_address;
        let data_len = data.len() as u64;
        self.tables.insert(signature.to_string(), data);
        self.next_address += data_len;
        // Align to 16-byte boundary
        self.next_address = (self.next_address + 15) & !15;
        address
    }

    pub fn get_table(&self, signature: &str) -> Option<&Vec<u8>> {
        self.tables.get(signature)
    }

    pub fn get_table_address(&self, signature: &str) -> Option<u64> {
        // RSDP has a fixed address
        if signature == "RSDP" {
            return Some(crate::acpi::RSDP_ADDRESS);
        }
        
        // Other tables are stored sequentially starting at ACPI_TABLES_BASE
        let mut address = crate::acpi::ACPI_TABLES_BASE;
        for (sig, data) in &self.tables {
            if sig == signature {
                return Some(address);
            }
            address += data.len() as u64;
            address = (address + 15) & !15; // Align to 16-byte boundary
        }
        None
    }

    pub fn get_all_tables(&self) -> &HashMap<String, Vec<u8>> {
        &self.tables
    }
}
