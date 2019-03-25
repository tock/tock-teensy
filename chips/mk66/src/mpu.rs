//! Implementation of the MK66 memory protection unit.
//!
//! This implementation relies on some hacks to work around the current
//! MPU interface, which is highly Cortex-M specific.
//!
//! Note that the current process.rs requests a grant region disallowing
//! user access overlapping a process memory region allowing full user access. 
//! On this MPU, this overlap gives the user full access to the grant region, 
//! which is unintended behaviour.
//!
//! - Author: Conor McAvity <cmcavity@stanford.edu>
//! - Updated to 1.3 MPU interface by Philip Levis <pal@cs.stanford.edu>

use kernel::common::registers::{register_bitfields, FieldValue, ReadOnly, ReadWrite};
use kernel::common::StaticRef;
use kernel::mpu;

const APP_MEMORY_REGION_NUM: usize = 0;

#[derive(Copy, Clone)]
pub struct K66Config {
    // There are 12 regions, but the first is reserved for the debugger
    regions: [K66ConfigRegion; 11],
}

#[repr(C)]
struct K66ErrorRegisters {
    ear: ReadOnly<u32, ErrorAddress::Register>,
    edr: ReadOnly<u32, ErrorDetail::Register>,
}


#[repr(C)]
#[derive(Clone, Copy)]
struct K66ConfigRegion {
    location: Option<(u32, u32)>,
    permissions: mpu::Permissions,
    super_as_user: bool,
    rgd_word0: FieldValue<u32, RegionDescriptorWord0::Register>,
    rgd_word1: FieldValue<u32, RegionDescriptorWord1::Register>,
    rgd_word2: FieldValue<u32, RegionDescriptorWord2::Register>,
    rgd_word3: FieldValue<u32, RegionDescriptorWord3::Register>,
}

/// Represention of the K66 MPU registers for one region
#[repr(C)]
struct K66RegionRegisters {
    rgd_word0: ReadWrite<u32, RegionDescriptorWord0::Register>,
    rgd_word1: ReadWrite<u32, RegionDescriptorWord1::Register>,
    rgd_word2: ReadWrite<u32, RegionDescriptorWord2::Register>,
    rgd_word3: ReadWrite<u32, RegionDescriptorWord3::Register>,
}



impl K66ConfigRegion {
    fn new(start: u32, end: u32, 
           permissions: mpu::Permissions, 
           super_as_user: bool) -> K66ConfigRegion {

        let user_val: u8 = match permissions {
            mpu::Permissions::ReadWriteExecute => 0b111,
            mpu::Permissions::ReadWriteOnly    => 0b110,
            mpu::Permissions::ReadExecuteOnly  => 0b101,
            mpu::Permissions::ReadOnly         => 0b100,
            mpu::Permissions::ExecuteOnly      => 0b001,
        };
        let super_val = if super_as_user { 0b11 } else { 0 };
        
        K66ConfigRegion {
            location: Some((start, end)),
            permissions: permissions,
            super_as_user: super_as_user,
            rgd_word0: RegionDescriptorWord0::SRTADDR.val(start >> 5),
            rgd_word1: RegionDescriptorWord1::ENDADDR.val(end >> 5),
            rgd_word2: RegionDescriptorWord2::M0SM.val(super_val) + 
                       RegionDescriptorWord2::M0UM.val(user_val as u32),
            rgd_word3: RegionDescriptorWord3::VLD::SET, 
        } 
    }

   
    fn empty() -> K66ConfigRegion {
        K66ConfigRegion {
            location: None,
            permissions: mpu::Permissions::ReadOnly,
            super_as_user: false,
            rgd_word0: RegionDescriptorWord0::SRTADDR::CLEAR, 
            rgd_word1: RegionDescriptorWord1::ENDADDR::CLEAR, 
            rgd_word2: RegionDescriptorWord2::M0UM::CLEAR, 
            rgd_word3: RegionDescriptorWord3::VLD::CLEAR, 
        }
    }

    fn overlaps(&self, start: *const u8, size: u32) -> bool {
        let region_start = self.base_address();
        let region_end = self.end_address();
        let start = start as u32;
        let end = start + size;
        start < region_end && end > region_start
    }
   
    fn location(&self) -> Option<(u32, u32)> {
        self.location
    }

    fn base_address(&self) -> u32 {
        self.location.map_or(0, |(start, _)| start)
    }

    fn end_address(&self) -> u32 {
        self.location.map_or(0, |(_, end)| end)
    }
  
    fn supervisor_as_user(&self) -> bool {
        self.super_as_user
    }
    
    fn user_permissions(&self) -> mpu::Permissions {
        self.permissions
    } 

}
#[repr(C)]
struct MpuAlternateAccessControl( 
    ReadWrite<u32, RegionDescriptorWord2::Register>
);

/// MPU registers for the K66
///
/// Described in section 22.4 of
/// <https://www.nxp.com/docs/en/reference-manual/K66P144M180SF5RMV2.pdf>
#[repr(C)]
struct MpuRegisters {
    cesr: ReadWrite<u32, ControlErrorStatus::Register>,
    _reserved0: [u32; 3],
    ers: [K66ErrorRegisters; 5],
    _reserved1: [u32; 242],
    rgds: [K66RegionRegisters; 12],
    _reserved2: [u32; 208],
    rgdaacs: [MpuAlternateAccessControl; 12],
}

register_bitfields![u32,
    ControlErrorStatus [
        /// Slave Port 0 Error
        SP0ERR OFFSET(31) NUMBITS(1) [],
        /// Slave Port 1 Error
        SP1ERR OFFSET(30) NUMBITS(1) [],
        /// Slave Port 2 Error
        SP2ERR OFFSET(29) NUMBITS(1) [],
        /// Slave Port 3 Error
        SP3ERR OFFSET(28) NUMBITS(1) [],
        /// Slave Port 4 Error
        SP4ERR OFFSET(27) NUMBITS(1) [],
        /// Hardware Revision Level
        HRL OFFSET(16) NUMBITS(4) [],
        /// Number Of Slave Ports
        NSP OFFSET(12) NUMBITS(4) [],
        /// Number Of Region Descriptors
        NRGD OFFSET(8) NUMBITS(4) [
            Eight = 0,
            Twelve = 1,
            Sixteen = 2
        ],
        /// Valid
        VLD OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ],

    ErrorAddress [
        /// Error Address
        EADDR OFFSET(0) NUMBITS(32) []
    ],

    ErrorDetail [
        /// Error Access Control Detail
        EACD OFFSET(16) NUMBITS(16) [],
        /// Error Process Identification
        EPID OFFSET(8) NUMBITS(8) [],
        /// Error Master Number
        EMN OFFSET(4) NUMBITS(4) [],
        /// Error Attributes
        EATTR OFFSET(1) NUMBITS(3) [
            UserModeInstructionAccess = 0,
            UserModeDataAccess = 1,
            SupervisorModeInstructionAccess = 2,
            SupervisorModeDataAccess = 3
        ],
        /// Error Read/Write
        ERW OFFSET(1) NUMBITS(1) [
            Read = 0,
            Write = 1
        ]
    ],

    RegionDescriptorWord0 [
        /// Start Address
        SRTADDR OFFSET(5) NUMBITS(27) []
    ],

    RegionDescriptorWord1 [
        /// End Address
        ENDADDR OFFSET(5) NUMBITS(27) []
    ],

    RegionDescriptorWord2 [
        /// Bus Master 7 Read Enable
        M7RE OFFSET(31) NUMBITS(1) [],
        /// Bus Master 7 Write Enable
        M7WE OFFSET(30) NUMBITS(1) [],
        /// Bus Master 6 Read Enable
        M6RE OFFSET(29) NUMBITS(1) [],
        /// Bus Master 6 Write Enable
        M6WE OFFSET(28) NUMBITS(1) [],
        /// Bus Master 5 Read Enable
        M5RE OFFSET(27) NUMBITS(1) [],
        /// Bus Master 5 Write Enable
        M5WE OFFSET(26) NUMBITS(1) [],
        /// Bus Master 4 Read Enable
        M4RE OFFSET(25) NUMBITS(1) [],
        /// Bus Master 4 Write Enable
        M4WE OFFSET(24) NUMBITS(1) [],
        /// Bus Master 3 Process Identifier Enable
        M3PE OFFSET(23) NUMBITS(1) [],
        /// Bus Master 3 Supervisor Mode Access Control
        M3SM OFFSET(21) NUMBITS(2) [
            ReadWriteExecute = 0,
            ReadExecuteOnly = 1,
            ReadWriteOnly = 2,
            SameAsUserMode = 3 
        ],
        /// Bus Master 3 User Mode Access Control
        M3UM OFFSET(18) NUMBITS(3) [],
        /// Bus Master 2 Process Identifier Enable
        M2PE OFFSET(17) NUMBITS(1) [],
        /// Bus Master 2 Supervisor Mode Access Control
        M2SM OFFSET(15) NUMBITS(2) [
            ReadWriteExecute = 0,
            ReadExecuteOnly = 1,
            ReadWriteOnly = 2,
            SameAsUserMode = 3 
        ],
        /// Bus Master 2 User Mode Access Control 
        M2UM OFFSET(12) NUMBITS(3) [],
        /// Bus Master 1 Process Identifier Enable
        M1PE OFFSET(11) NUMBITS(1) [],
        /// Bus Master 1 Supervisor Mode Access Control
        M1SM OFFSET(9) NUMBITS(2) [
            ReadWriteExecute = 0,
            ReadExecuteOnly = 1,
            ReadWriteOnly = 2,
            SameAsUserMode = 3 
        ],
        /// Bus Master 1 User Mode Access Control
        M1UM OFFSET(6) NUMBITS(3) [],
        /// Bus Master 0 Process Identifier Enable
        M0PE OFFSET(5) NUMBITS(1) [],
        /// Bus Master 0 Supervisor Mode Access Control
        M0SM OFFSET(3) NUMBITS(2) [
            ReadWriteExecute = 0,
            ReadExecuteOnly = 1,
            ReadWriteOnly = 2,
            SameAsUserMode = 3 
        ],
        /// Bus Master 0 User Mode Access Control 
        M0UM OFFSET(0) NUMBITS(3) []
    ],

    RegionDescriptorWord3 [
        /// Process Identifier
        PID OFFSET(24) NUMBITS(8) [],
        /// Process Identifier Mask
        PIDMASK OFFSET(16) NUMBITS(8) [],
        /// Valid
        VLD OFFSET(0) NUMBITS(1) []
    ]
];

const BASE_ADDRESS: StaticRef<MpuRegisters> =
    unsafe { StaticRef::new(0x4000D000 as *const MpuRegisters) };

pub struct K66Mpu(StaticRef<MpuRegisters>);

impl K66Mpu {
    pub const unsafe fn new () -> K66Mpu {
        K66Mpu(BASE_ADDRESS)
    }
}

impl K66Config {
    fn unused_region_number(&self) -> Option<usize> {
        for (number, region) in self.regions.iter().enumerate() {
            if number == APP_MEMORY_REGION_NUM {
                continue;
            }
            if let None = region.location() {
                return Some(number);
             }
        }
        None
    }
}

impl Default for K66Config {
    fn default() -> K66Config {
        K66Config {
            regions: [
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
		K66ConfigRegion::empty(),
                K66ConfigRegion::empty(),
            ],
        }
    }
}

impl mpu::MPU for K66Mpu {
    type MpuConfig = K66Config;

    fn enable_mpu(&self) {
        let regs = &*self.0;

        // On reset, region descriptor 0 is allocated to give full access to 
        // the entire 4 GB memory space to the core in both supervisor and user
        // mode, so we disable access for user mode
        regs.rgdaacs[0].0.modify(RegionDescriptorWord2::M0SM::ReadWriteExecute);
        regs.rgdaacs[0].0.modify(RegionDescriptorWord2::M0UM::CLEAR);

        regs.cesr.modify(ControlErrorStatus::VLD::Enable);
    }    
    
    fn disable_mpu(&self) {
        let regs = &*self.0;
        regs.cesr.modify(ControlErrorStatus::VLD::Disable);
    }

    fn allocate_region(
        &self,
        unallocated_memory_start: *const u8,
        unallocated_memory_size: usize,
        min_region_size: usize,
        access: mpu::Permissions,
        config: &mut Self::MpuConfig,
    ) -> Option<mpu::Region> {
        for region in config.regions.iter() {
            if region.overlaps(unallocated_memory_start, unallocated_memory_size as u32) {
                return None;
            }
        }

        let region_num = config.unused_region_number()?;

        let mut start = unallocated_memory_start as u32;

        // We only have 12 region descriptors, and regions must be 32-byte aligned
        if region_num > 11 || 
           start % 32 != 0 || 
           unallocated_memory_size % 32 != 0 {
            return None;
        }

        let unallocated_memory_size: u32 = unallocated_memory_size as u32;
        // The end address register is always 31 modulo 32
        let end = ((start + unallocated_memory_size - 1) & !0x1f) as u32;

        // Allocate a new region with these permissions and supervisor has full
        // permissions.
        let region = unsafe { K66ConfigRegion::new(start, end, access, false) };
        config.regions[region_num] = region;
        let start_addr = unsafe {start as *const u8};
        let size: usize = (end - start) as usize;
        let region = mpu::Region::new(start_addr, size);
        Some(region)
    }

    fn number_total_regions(&self) -> usize {
        11   // There are 12, but region 0 is reserved for debugger
    }

    fn allocate_app_memory_region(
        &self,
        unallocated_memory_start: *const u8,
        unallocated_memory_size: usize,
        min_memory_size: usize,
        initial_app_memory_size: usize,
        initial_kernel_memory_size: usize,
        permissions: mpu::Permissions,
        config: &mut Self::MpuConfig,
    ) -> Option<(*const u8, usize)> {

    }

    fn update_app_memory_region(
        &self,
        app_memory_break: *const u8,
        kernel_memory_break: *const u8,
        permissions: mpu::Permissions,
        config: &mut Self::MpuConfig,
    ) -> Result<(), ()> {
  
    }

    fn configure_mpu(&self, config: &Self::MpuConfig) {
        let regs = &*self.0;
        for (i, region) in config.regions.iter().enumerate() {
            let base_address = u32::from(region.base_address());
            let end_address = u32::from(region.end_address());

            let permissions = region.user_permissions();
            let user: u32 = match permissions {
                mpu::Permissions::ReadWriteExecute => 0b111,
                mpu::Permissions::ReadWriteOnly    => 0b110,
                mpu::Permissions::ReadExecuteOnly  => 0b101,
                mpu::Permissions::ReadOnly         => 0b100,
                mpu::Permissions::ExecuteOnly      => 0b001,
            };
            
            let super_as_user = region.supervisor_as_user();
            let supervisor = if super_as_user {0b11} else {0b00};
            
            let start = base_address >> 5; 
            let end = end_address >> 5;

            // Add 1 because region 0 is reserved. The 11 regions
            // with i=0..10 refer to regions 1.11.
            let region_num = i + 1; 
            // Write to region descriptor
            regs.rgds[i].rgd_word0.set(RegionDescriptorWord0::SRTADDR.val(start));
            regs.rgds[i].rgd_word1.set(RegionDescriptorWord1::ENDADDR.val(end));
            regs.rgds[i].rgd_word2.write(RegionDescriptorWord2::M3UM.val(user));
            regs.rgds[i].rgd_word2.write(RegionDescriptorWord2::M3SM.val(supervisor));
            regs.rgds[i].rgd_word3.write(RegionDescriptorWord3::VLD::SET);
        }
    }
}
