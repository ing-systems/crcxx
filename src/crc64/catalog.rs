use super::Params;

pub const CRC_40_GSM: Params<u64> = Params { width: 40, poly: 0x0004820009, init: 0x0000000000, refin: false, refout: false, xorout: 0xffffffffff, check: 0xd4164fc646, residue: 0xc4ff8071ff };
pub const CRC_64_ECMA_182: Params<u64> = Params { width: 64, poly: 0x42f0e1eba9ea3693, init: 0x0000000000000000, refin: false, refout: false, xorout: 0x0000000000000000, check: 0x6c40df5f0b497347, residue: 0x0000000000000000 };
pub const CRC_64_GO_ISO: Params<u64> = Params { width: 64, poly: 0x000000000000001b, init: 0xffffffffffffffff, refin: true, refout: true, xorout: 0xffffffffffffffff, check: 0xb90956c775a41001, residue: 0x5300000000000000 };
pub const CRC_64_MS: Params<u64> = Params { width: 64, poly: 0x259c84cba6426349, init: 0xffffffffffffffff, refin: true, refout: true, xorout: 0x0000000000000000, check: 0x75d4b74f024eceea, residue: 0x0000000000000000 };
pub const CRC_64_REDIS: Params<u64> = Params { width: 64, poly: 0xad93d23594c935a9, init: 0x0000000000000000, refin: true, refout: true, xorout: 0x0000000000000000, check: 0xe9c6d914c4b8d9ca, residue: 0x0000000000000000 };
pub const CRC_64_WE: Params<u64> = Params { width: 64, poly: 0x42f0e1eba9ea3693, init: 0xffffffffffffffff, refin: false, refout: false, xorout: 0xffffffffffffffff, check: 0x62ec59e3f1a4f00a, residue: 0xfcacbebd5931a992 };
pub const CRC_64_XZ: Params<u64> = Params { width: 64, poly: 0x42f0e1eba9ea3693, init: 0xffffffffffffffff, refin: true, refout: true, xorout: 0xffffffffffffffff, check: 0x995dc9bbdf1939fa, residue: 0x49958c9abd7d353f };
