// This is a very small subset of open aes.
// No pointers, no problems.
//
// References:
// https://github.com/jhjin/OpenAES
// https://github.com/noahdesu/xmonarch/blob/master/oaes_lib.h
// https://github.com/noahdesu/xmonarch/blob/master/oaes_lib.c

//
//  Constants
//

const OAES_RKEY_LEN     : usize = 4;
const OAES_COL_LEN      : usize = 4;

static OAES_GF_8 : [u8; 10] = [
    0x01, 0x02, 0x04, 0x08, 0x10,
    0x20, 0x40, 0x80, 0x1b, 0x36
];

static OAES_SUB_BYTE_VALUE : [[u8; 16]; 16] = [
    //           0,    1,    2,    3,    4,    5,    6,    7,    8,    9,    a,    b,    c,    d,    e,    f,
    /* 0 */    [ 0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76 ],
    /* 1 */    [ 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0 ],
    /* 2 */    [ 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15 ],
    /* 3 */    [ 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75 ],
    /* 4 */    [ 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84 ],
    /* 5 */    [ 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf ],
    /* 6 */    [ 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8 ],
    /* 7 */    [ 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2 ],
    /* 8 */    [ 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73 ],
    /* 9 */    [ 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb ],
    /* a */    [ 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79 ],
    /* b */    [ 0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08 ],
    /* c */    [ 0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a ],
    /* d */    [ 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e ],
    /* e */    [ 0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf ],
    /* f */    [ 0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16 ],
];

//
// Structures
//

pub struct AesKey {
    pub data: Vec<u8>,
    pub data_len: usize,
    pub exp_data: Vec<u8>,
    pub exp_data_len: usize,
    pub num_keys: usize,
    pub key_base: usize,
}

pub struct AesContext {
    pub key: AesKey,
}

//
// Functions
//

fn oaes_word_rot_left(word: &mut[u8; OAES_COL_LEN]) {
    let mut temp = [0u8; OAES_COL_LEN];
    // memcpy( _temp, word + 1, OAES_COL_LEN - 1 );
    for i in 0..OAES_COL_LEN - 1 {
        temp[i] = word[i + 1];
    }
    // _temp[OAES_COL_LEN - 1] = word[0];
    temp[OAES_COL_LEN - 1] = word[0];
    // memcpy( word, _temp, OAES_COL_LEN );
    for i in 0..OAES_COL_LEN {
        word[i] = temp[i];
    }
}

fn oaes_sub_byte(byte: &mut u8) {
    // _y = ((_x = *byte) >> 4) & 0x0f;
    let y = (*byte >> 4) & 0x0f;
    // _x &= 0x0f;
    let x = *byte & 0x0f;
    // *byte = oaes_sub_byte_value[_y][_x];
    *byte = OAES_SUB_BYTE_VALUE[y as usize][x as usize];
}

//
// Implementations
//

impl AesContext {
    pub fn import_key_data(&mut self, data: &[u8], data_len: usize) {
        self.key = AesKey::default();
        self.key.data_len = data_len;
        self.key.data = vec![0u8; data_len];
        // memcpy( _ctx->key->data, data, data_len );
        for i in 0..usize::min(data.len(), data_len) {
            self.key.data[i] = data[i];
        }
        // oaes_key_expand( ctx );
        self.key_expand();
    }

    fn key_expand(&mut self) {
        self.key.key_base = 8;
        self.key.num_keys = 15;
        self.key.exp_data_len = 240;
        // _ctx->key->exp_data = (uint8_t *)calloc( _ctx->key->exp_data_len, sizeof( uint8_t ));
        self.key.exp_data = vec![0u8; self.key.exp_data_len];

        // memcpy( _ctx->key->exp_data, _ctx->key->data, _ctx->key->data_len );
        for i in 0..self.key.data_len {
            self.key.exp_data[i] = self.key.data[i];
        }

        // Expand algorithm
        // for(_i = 8; _i < 60; _i++)
        for i in 8..60 {
            // uint8_t _temp[OAES_COL_LEN];
            let mut temp = [0u8; OAES_COL_LEN];
            // memcpy( _temp, _ctx->key->exp_data + ( _i - 1 ) * OAES_RKEY_LEN, OAES_COL_LEN );
            for col in 0..OAES_COL_LEN {
                temp[col] = self.key.exp_data[((i - 1) * OAES_RKEY_LEN) + col];
            }
            if i % 8 == 0 {
                // oaes_word_rot_left( _temp );
                oaes_word_rot_left(&mut temp);
                // for( _j = 0; _j < OAES_COL_LEN; _j++ )
                for col in 0..OAES_COL_LEN {
                    // oaes_sub_byte( _temp + _j );
                    oaes_sub_byte(&mut temp[col]);
                }
                // _temp[0] = _temp[0] ^ oaes_gf_8[ _i / _ctx->key->key_base - 1 ];
                temp[0] ^= OAES_GF_8[i / self.key.key_base - 1];
            } else if i % self.key.key_base == 4 {
                // for( _j = 0; _j < OAES_COL_LEN; _j++ )
                for col in 0..OAES_COL_LEN {
                    // oaes_sub_byte( _temp + _j );
                    oaes_sub_byte(&mut temp[col]);
                }
            }
            // for( _j = 0; _j < OAES_COL_LEN; _j++ )
            for j in 0..OAES_COL_LEN {
                // _ctx->key->exp_data[ _i * OAES_RKEY_LEN + _j ] =
                //    _ctx->key->exp_data[ ( _i - _ctx->key->key_base ) *
                // OAES_RKEY_LEN + _j ] ^ _temp[_j];
                let index = (i - self.key.key_base) * OAES_RKEY_LEN + j;
                self.key.exp_data[i * OAES_RKEY_LEN + j] =
                    self.key.exp_data[index] ^ temp[j];
            }
        }
    }
}

impl Default for AesKey {
    fn default() -> AesKey {
        unsafe { ::std::mem::zeroed::<AesKey>() }
    }
}

impl Default for AesContext {
    fn default() -> AesContext {
        unsafe { ::std::mem::zeroed::<AesContext>() }
    }
}