use crc::{Crc, CRC_16_XMODEM};

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

pub const HASH_SLOT_MAX: u16 = 16384;

pub const fn crc_hash(bytes: &[u8]) -> u16 {
    CRC.checksum(bytes) % HASH_SLOT_MAX
}
