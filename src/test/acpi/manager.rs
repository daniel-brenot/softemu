use crate::acpi::AcpiManager;
use crate::memory::GuestMemory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_manager_creation() {
        let manager = AcpiManager::new();
        assert!(!manager.is_initialized());
    }

    #[test]
    fn test_acpi_manager_initialization() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        
        let result = manager.initialize(memory);
        assert!(result.is_ok());
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_acpi_powerstates() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test initial power state
        let initialstate = manager.get_powerstate();
        assert_eq!(initialstate, crate::acpi::AcpiPowerState::S0);
        
        // Test power state transition
        let result = manager.transition_powerstate(crate::acpi::AcpiPowerState::S3);
        assert!(result.is_ok());
        assert_eq!(manager.get_powerstate(), crate::acpi::AcpiPowerState::S3);
    }

    #[test]
    fn test_acpi_power_button() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test power button press
        let result = manager.handle_power_button();
        assert!(result.is_ok());
        
        // Should transition to S5 (soft off)
        assert_eq!(manager.get_powerstate(), crate::acpi::AcpiPowerState::S5);
    }

    #[test]
    fn test_acpi_sleep_button() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test sleep button press
        let result = manager.handle_sleep_button();
        assert!(result.is_ok());
        
        // Should transition to S3 (suspend to RAM)
        assert_eq!(manager.get_powerstate(), crate::acpi::AcpiPowerState::S3);
    }

    #[test]
    fn test_acpi_lid_switch() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test lid close
        let result = manager.handle_lid_switch(true);
        assert!(result.is_ok());
        
        // Should transition to S3 (suspend to RAM)
        assert_eq!(manager.get_powerstate(), crate::acpi::AcpiPowerState::S3);
        
        // Test lid open
        let result = manager.handle_lid_switch(false);
        assert!(result.is_ok());
        
        // Should transition back to S0 (working)
        assert_eq!(manager.get_powerstate(), crate::acpi::AcpiPowerState::S0);
    }

    #[test]
    fn test_acpi_power_consumption() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test power consumption in S0
        let consumption = manager.get_power_consumption();
        assert_eq!(consumption, 100.0); // Full power
        
        // Transition to S3 and test power consumption
        manager.transition_powerstate(crate::acpi::AcpiPowerState::S3).unwrap();
        let consumption = manager.get_power_consumption();
        assert_eq!(consumption, 5.0); // Minimal power
    }

    #[test]
    fn test_acpi_table_creation() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test that tables are created
        assert!(manager.get_table("RSDP").is_some());
        assert!(manager.get_table("RSDT").is_some());
        assert!(manager.get_table("XSDT").is_some());
        assert!(manager.get_table("FACP").is_some());
        assert!(manager.get_table("APIC").is_some());
        assert!(manager.get_table("MCFG").is_some());
    }

    #[test]
    fn test_acpi_table_addresses() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        // Test that table addresses are valid
        let rsdp_addr = manager.get_table_address("RSDP");
        assert!(rsdp_addr.is_some());
        assert_eq!(rsdp_addr.unwrap(), crate::acpi::RSDP_ADDRESS);
        
        let rsdt_addr = manager.get_table_address("RSDT");
        assert!(rsdt_addr.is_some());
        assert!(rsdt_addr.unwrap() >= crate::acpi::ACPI_TABLES_BASE);
    }

    #[test]
    fn test_acpi_shutdown() {
        let mut manager = AcpiManager::new();
        let memory = GuestMemory::new(16 * 1024 * 1024).unwrap(); // 16MB to accommodate ACPI tables
        manager.initialize(memory).unwrap();
        
        assert!(manager.is_initialized());
        
        let result = manager.shutdown();
        assert!(result.is_ok());
        assert!(!manager.is_initialized());
    }
}
