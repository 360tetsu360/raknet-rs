use raknet::{packets::*, reader::Reader, writer::Writer};

const UNCONNECTED_PING_DATA: [u8; 33] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x3d, 0x64, 0x94, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe,
    0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78, 0x8c, 0xe5, 0xbf, 0x69, 0x32, 0xf8, 0x7a,
    0x55,
];

const UNCONNECTED_PONG_DATA: [u8; 132] = [
    0x1c, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x42, 0x80, 0xb5, 0xa2, 0xd6, 0x93, 0xa7, 0x1e, 0x81, 0x85,
    0xa2, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0x00, 0x61, 0x4d, 0x43, 0x50, 0x45, 0x3b, 0x44, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x3b, 0x34, 0x37, 0x31, 0x3b, 0x31, 0x2e, 0x31,
    0x37, 0x2e, 0x34, 0x31, 0x3b, 0x30, 0x3b, 0x31, 0x30, 0x3b, 0x31, 0x31, 0x37, 0x33, 0x33, 0x37,
    0x32, 0x38, 0x32, 0x32, 0x35, 0x31, 0x34, 0x31, 0x30, 0x33, 0x32, 0x33, 0x35, 0x34, 0x3b, 0x42,
    0x65, 0x64, 0x72, 0x6f, 0x63, 0x6b, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x3b, 0x53, 0x75, 0x72,
    0x76, 0x69, 0x76, 0x61, 0x6c, 0x3b, 0x31, 0x3b, 0x31, 0x39, 0x31, 0x33, 0x32, 0x3b, 0x31, 0x39,
    0x31, 0x33, 0x33, 0x3b,
];

const OPEN_CONNECTION_REQUEST1_DATA: [u8; 1464] = [
    0x05, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const OPEN_CONNECTION_REPLY1_DATA: [u8; 28] = [
    0x06, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0xa2, 0xd6, 0x93, 0xa7, 0x1e, 0x81, 0x85, 0xa2, 0x00, 0x05, 0x78,
];

const OPEN_CONNECTION_REQUEST2_DATA: [u8; 34] = [
    0x07, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0x04, 0x80, 0xff, 0xff, 0xfe, 0x4a, 0xbc, 0x05, 0x78, 0x8a, 0xea, 0x74, 0x25, 0xff, 0xd6,
    0x14, 0x0f,
];

const OPEN_CONNECTION_REPLY2_DATA: [u8; 35] = [
    0x08, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0xa2, 0xd6, 0x93, 0xa7, 0x1e, 0x81, 0x85, 0xa2, 0x04, 0x80, 0xff, 0xff, 0xfe, 0xe2, 0xbe,
    0x05, 0x78, 0x00,
];

const CONNECTION_REQUEST_DATA: [u8; 18] = [
    0x09, 0x8a, 0xea, 0x74, 0x25, 0xff, 0xd6, 0x14, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x42, 0xab,
    0xec, 0x00,
];

const CONNECTION_REQUEST_ACCEPTED_DATA: [u8; 606] = [
    0x10, 0x04, 0x80, 0xff, 0xff, 0xfe, 0xfb, 0x57, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
    0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x06, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x19, 0x4f, 0xc3, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x61, 0x72, 0x86, 0x7c,
];

const NEW_INCOMING_CONNECTION_DATA: [u8; 198] = [
    0x13, 0x04, 0xcc, 0xb0, 0x1b, 0xf4, 0x4a, 0xbc, 0x06, 0x17, 0x00, 0xf1, 0x03, 0x00, 0x00, 0x00,
    0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xa0, 0xd7, 0xcd, 0x26, 0xb0, 0x2b,
    0xc2, 0x02, 0x00, 0x00, 0x00, 0x04, 0x3f, 0x57, 0xff, 0xef, 0xf1, 0x03, 0x04, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00,
    0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04,
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff,
    0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
    0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff,
    0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x8d, 0xcc, 0x27, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x0f, 0x8d, 0xcc, 0x27,
];

const INCOMPATIBLE_PROTOCOL_VERSION_DATA: [u8; 26] = [
    0x19, 0x0a, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
    0x56, 0x78, 0xa2, 0xa8, 0x7f, 0xa9, 0xae, 0xe7, 0xe2, 0x6b,
];
const ACK_DATA: [u8; 7] = [0xc0, 0x00, 0x01, 0x01, 0x02, 0x00, 0x00];
const ACK_DATA2: [u8; 10] = [0xc0, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00];
const NACK_DATA: [u8; 7] = [0xa0, 0x00, 0x01, 0x01, 0x0d, 0x00, 0x00];
const NACK_DATA2: [u8; 10] = [0xa0, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00];

const CONNECTEDPING_DATA: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x4f, 0xc3, 0xe1];

const CONNECTEDPONG_DATA: [u8; 17] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x19, 0x4f, 0xc3, 0xe1, 0x00, 0x00, 0x00, 0x00, 0x61, 0x72, 0x86,
    0x7c,
];

const FRAME_SETPACKET_DATA: [u8; 212] = [
    0x84, 0x01, 0x00, 0x00, 0x60, 0x05, 0xd0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x04,
    0x80, 0xff, 0xff, 0xfe, 0x4a, 0xbc, 0x06, 0x17, 0x00, 0xfb, 0x57, 0x00, 0x00, 0x00, 0x00, 0xfe,
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x35, 0xe2, 0xe6, 0xe0, 0xc1, 0x68, 0x88, 0x06,
    0x00, 0x00, 0x00, 0x04, 0x3f, 0x57, 0xff, 0xee, 0xfb, 0x57, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00,
    0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04,
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff,
    0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
    0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff,
    0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00,
    0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x72, 0x86, 0x7c,
    0x00, 0x00, 0x00, 0x00, 0x19, 0x4f, 0xc3, 0xd1, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x19, 0x4f, 0xc3, 0xd2,
];

const ALREADY_CONNECTED_DATA: [u8; 25] = [
    0x12, 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, 0x91, 0x1b, 0x13, 0x5d, 0x5f, 0x63, 0x9d, 0x1f,
];

#[tokio::test]
async fn raknet_packet() {
    let unconnected_ping = decode::<UnconnectedPing>(&UNCONNECTED_PING_DATA)
        .await
        .unwrap();
    let unconnected_ping_encoded = encode::<UnconnectedPing>(unconnected_ping).await.unwrap();
    debug_assert_eq!(&unconnected_ping_encoded, &UNCONNECTED_PING_DATA);

    let unconnected_pong = decode::<UnconnectedPong>(&UNCONNECTED_PONG_DATA)
        .await
        .unwrap();
    let unconnected_pong_encoded = encode::<UnconnectedPong>(unconnected_pong).await.unwrap();
    debug_assert_eq!(&unconnected_pong_encoded, &UNCONNECTED_PONG_DATA);

    let open_connection_request1 = decode::<OpenConnectionRequest1>(&OPEN_CONNECTION_REQUEST1_DATA)
        .await
        .unwrap();
    let open_connection_request1_encoded =
        encode::<OpenConnectionRequest1>(open_connection_request1)
            .await
            .unwrap();
    debug_assert_eq!(
        &open_connection_request1_encoded,
        &OPEN_CONNECTION_REQUEST1_DATA
    );

    let open_connection_request2 = decode::<OpenConnectionRequest2>(&OPEN_CONNECTION_REQUEST2_DATA)
        .await
        .unwrap();
    let open_connection_request2_encoded =
        encode::<OpenConnectionRequest2>(open_connection_request2)
            .await
            .unwrap();
    debug_assert_eq!(
        &open_connection_request2_encoded,
        &OPEN_CONNECTION_REQUEST2_DATA
    );

    let open_connection_reply1 = decode::<OpenConnectionReply1>(&OPEN_CONNECTION_REPLY1_DATA)
        .await
        .unwrap();
    let open_connection_reply1_encoded = encode::<OpenConnectionReply1>(open_connection_reply1)
        .await
        .unwrap();
    debug_assert_eq!(
        &open_connection_reply1_encoded,
        &OPEN_CONNECTION_REPLY1_DATA
    );

    let open_connection_reply2 = decode::<OpenConnectionReply2>(&OPEN_CONNECTION_REPLY2_DATA)
        .await
        .unwrap();
    let open_connection_reply2_encoded = encode::<OpenConnectionReply2>(open_connection_reply2)
        .await
        .unwrap();
    debug_assert_eq!(
        &open_connection_reply2_encoded,
        &OPEN_CONNECTION_REPLY2_DATA
    );

    let connection_request = decode::<ConnectionRequest>(&CONNECTION_REQUEST_DATA)
        .await
        .unwrap();
    let connection_request_encoded = encode::<ConnectionRequest>(connection_request)
        .await
        .unwrap();
    debug_assert_eq!(&connection_request_encoded, &CONNECTION_REQUEST_DATA);

    let connection_request_accepted =
        decode::<ConnectionRequestAccepted>(&CONNECTION_REQUEST_ACCEPTED_DATA)
            .await
            .unwrap();
    encode::<ConnectionRequestAccepted>(connection_request_accepted)
        .await
        .unwrap();

    let new_incoming_connection = decode::<NewIncomingConnection>(&NEW_INCOMING_CONNECTION_DATA)
        .await
        .unwrap();
    encode::<NewIncomingConnection>(new_incoming_connection)
        .await
        .unwrap();

    let incompatible_protocol_version =
        decode::<IncompatibleProtocolVersion>(&INCOMPATIBLE_PROTOCOL_VERSION_DATA)
            .await
            .unwrap();
    encode::<IncompatibleProtocolVersion>(incompatible_protocol_version)
        .await
        .unwrap();

    let ack = decode::<Ack>(&ACK_DATA).await.unwrap();
    let ack_encoded = encode::<Ack>(ack).await.unwrap();
    debug_assert_eq!(&ack_encoded, &ACK_DATA);

    let ack2 = decode::<Ack>(&ACK_DATA2).await.unwrap();
    let ack2_encoded = encode::<Ack>(ack2).await.unwrap();
    debug_assert_eq!(&ack2_encoded, &ACK_DATA2);

    let nack = decode::<Nack>(&NACK_DATA).await.unwrap();
    let nack_encoded = encode::<Nack>(nack).await.unwrap();
    debug_assert_eq!(&nack_encoded, &NACK_DATA);

    let nack2 = decode::<Nack>(&NACK_DATA2).await.unwrap();
    let nack2_encoded = encode::<Nack>(nack2).await.unwrap();
    debug_assert_eq!(&nack2_encoded, &NACK_DATA2);

    let connected_ping = decode::<ConnectedPing>(&CONNECTEDPING_DATA).await.unwrap();
    let connected_ping_encoded = encode::<ConnectedPing>(connected_ping).await.unwrap();
    debug_assert_eq!(&connected_ping_encoded, &CONNECTEDPING_DATA);

    let connected_pong = decode::<ConnectedPong>(&CONNECTEDPONG_DATA).await.unwrap();
    let connected_pong_encoded = encode::<ConnectedPong>(connected_pong).await.unwrap();
    debug_assert_eq!(&connected_pong_encoded, &CONNECTEDPONG_DATA);

    let frameset = FrameSet::decode(&FRAME_SETPACKET_DATA).await.unwrap();
    let frameset_encoded = frameset.encode().await.unwrap();
    debug_assert_eq!(&frameset_encoded, &FRAME_SETPACKET_DATA);

    let already_connected = decode::<AlreadyConnected>(&ALREADY_CONNECTED_DATA)
        .await
        .unwrap();
    let already_connected_encoded = encode::<AlreadyConnected>(already_connected).await.unwrap();
    debug_assert_eq!(&already_connected_encoded, &ALREADY_CONNECTED_DATA);

    let nack = Nack::new((0, 1));
    debug_assert_eq!(nack.get_all(), vec![0, 1]);

    let _nack_max_eq_min = Nack::new((0, 0));

    let _already_connected = AlreadyConnected::new(0);

    let _incompatible_protocol_version = IncompatibleProtocolVersion::new(0x0, 0x0);

    let sequenced_frame = Frame::new(Reliability::ReliableSequenced, b"test");
    let mut frame_buff = Writer::new(vec![]);
    sequenced_frame.encode(&mut frame_buff).await.unwrap();
    let buff = frame_buff.get_raw_payload();
    let mut reader = Reader::new(&buff);
    let _sequenced_frame_decoded = Frame::decode(&mut reader).await.unwrap();

    let disconnected = decode::<Disconnected>(&[0x15]).await.unwrap();
    debug_assert_eq!(&encode(disconnected).await.unwrap(), &[0x15]);
}

async fn get_encoded_len(f: Frame) -> usize {
    let mut data = Writer::new(vec![]);
    f.encode(&mut data).await.unwrap();
    data.get_raw_payload().len()
}

#[tokio::test]
async fn frame() {
    let frame = Frame::new(Reliability::Unreliable, &[0u8; 10]);
    debug_assert_eq!(frame.length(), get_encoded_len(frame).await);
    let frame = Frame::new(Reliability::UnreliableSequenced, &[0u8; 10]);
    debug_assert_eq!(frame.length(), get_encoded_len(frame).await);
    let frame = Frame::new(Reliability::Reliable, &[0u8; 10]);
    debug_assert_eq!(frame.length(), get_encoded_len(frame).await);
    let frame = Frame::new(Reliability::ReliableOrdered, &[0u8; 10]);
    debug_assert_eq!(frame.length(), get_encoded_len(frame).await);
    let frame = Frame::new(Reliability::ReliableSequenced, &[0u8; 10]);
    debug_assert_eq!(frame.length(), get_encoded_len(frame).await);
}

#[test]
fn event_error() {}
