use std::{convert::TryFrom, fmt::Display};

#[derive(Debug)]
pub enum Destination {
    Japan = 0x00,
    Overseas = 0x01,
    Invalid,
}

impl From<u8> for Destination {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Japan,
            0x01 => Self::Overseas,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly = 0x00,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    MBC2 = 0x05,
    Mbc2Battery = 0x06,
    MBC5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
}

impl TryFrom<u8> for CartridgeType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::RomOnly,
            0x08 => Self::RomRam,
            0x09 => Self::RomRamBattery,
            0x01 => Self::Mbc1,
            0x02 => Self::Mbc1Ram,
            0x03 => Self::Mbc1RamBattery,
            0x05 => Self::MBC2,
            0x06 => Self::Mbc2Battery,
            0x19 => Self::MBC5,
            0x1A => Self::Mbc5Ram,
            0x1B => Self::Mbc5RamBattery,
            _ => return Err(value),
        })
    }
}

#[derive(Debug)]
pub enum CartridgeWarning {
    InvalidTitle,
    InvalidNintendoHeader,
    InvalidLicencing(u8),
    InvalidDestination(u8),
    BadGlobalChecksum { expected: u16, computed: u16 },
}

#[derive(Debug)]
pub struct Cartridge<'rom> {
    data: &'rom [u8],
    title: &'rom str,
    licencee_code: u8,
    destination_code: Destination,
    pub warnings: Vec<CartridgeWarning>,
}

impl Display for Cartridge<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game: {}\nDestination: {:?}",
            self.title, self.destination_code
        )
    }
}

#[derive(Debug)]
pub enum CartridgeError {
    InvalidCartridgeType(u8),
    BadHeaderChecksum { expected: u8, computed: u8 },
}

#[derive(Debug)]
pub struct CartridgeErrors {
    errors: Vec<CartridgeError>,
}

impl<'rom> Cartridge<'rom> {
    pub fn load(rom: &'rom [u8]) -> Result<Self, CartridgeErrors> {
        let mut errors: Vec<CartridgeError> = Vec::new();
        let mut warnings: Vec<CartridgeWarning> = Vec::new();

        let cartrige_title_header = &rom[0x0134..0x0143];
        let title =
            str::from_utf8(cartrige_title_header).expect("rom should expect a valid utf8 title");

        let destination: Destination = rom[0x014A].into();

        let cartridge_type: Result<CartridgeType, u8> = rom[0x147].try_into();
        if let Err(invalid_type) = cartridge_type {
            errors.push(CartridgeError::InvalidCartridgeType(invalid_type));
        }

        //Verify that the nintendo logo is present in the cartridge header
        //this is not stricly required to have a valid rom but a conformant cartridge should have this valid header
        let nintendo_header: &[u8] = &[
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        let rom_nintendo_header = &rom[0x0104..=0x0133];

        if rom_nintendo_header != nintendo_header {
            warnings.push(CartridgeWarning::InvalidNintendoHeader);
        }

        //Verify header checksum and global checksum
        //invalid header checksum is treated as an error
        //invalid global checksum is treated as warning (the console didn't check it)
        let cartridge_header_checksum = rom[0x014D];
        let mut compute_header_checksum: u8 = 0;
        //verify checksum
        for b in &rom[0x0134..=0x014C] {
            compute_header_checksum = compute_header_checksum.wrapping_sub(*b);
            compute_header_checksum = compute_header_checksum.wrapping_sub(1);
        }

        if cartridge_header_checksum != compute_header_checksum {
            errors.push(CartridgeError::BadHeaderChecksum {
                expected: cartridge_header_checksum,
                computed: compute_header_checksum,
            });
        }

        let cartridge_global_checksum = u16::from_be_bytes([rom[0x014E], rom[0x014F]]);
        let mut compute_global_checksum: u16 = 0;
        for b in rom {
            compute_global_checksum = compute_global_checksum.wrapping_add((*b) as u16);
        }
        compute_global_checksum = compute_global_checksum.wrapping_sub(rom[0x014E] as u16);
        compute_global_checksum = compute_global_checksum.wrapping_sub(rom[0x014F] as u16);

        if cartridge_global_checksum != compute_global_checksum {
            warnings.push(CartridgeWarning::BadGlobalChecksum {
                expected: cartridge_global_checksum,
                computed: compute_global_checksum,
            });
        }

        if !errors.is_empty() {
            Err(CartridgeErrors { errors: errors })
        } else {
            Ok(Cartridge {
                data: rom,
                title: title,
                licencee_code: 0,
                destination_code: destination,
                warnings: warnings,
            })
        }
    }
}
