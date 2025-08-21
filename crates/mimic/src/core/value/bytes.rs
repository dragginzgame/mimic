use crate::core::value::Value;
use mimic_common::utils::hash::Xxh3;

///
/// Canonical Byte Representation
///

#[inline]
fn feed_u8(h: &mut Xxh3, x: u8) {
    h.update(&[x]);
}
#[inline]
fn feed_u32(h: &mut Xxh3, x: u32) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_u64(h: &mut Xxh3, x: u64) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_i64(h: &mut Xxh3, x: i64) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_bytes(h: &mut Xxh3, b: &[u8]) {
    h.update(b);
}

#[allow(clippy::cast_possible_truncation)]
impl Value {
    fn write_to_hasher(&self, h: &mut Xxh3) {
        feed_u8(h, self.tag());

        match self {
            Self::BigInt(v) => {
                feed_bytes(h, &v.to_leb128());
            }
            Self::BigUint(v) => {
                feed_bytes(h, &v.to_leb128());
            }
            Self::Blob(v) => {
                feed_u8(h, 0x01);
                feed_bytes(h, v);
            }
            Self::Bool(b) => {
                feed_u8(h, u8::from(*b));
            }
            Self::Decimal(d) => {
                // encode (sign, scale, mantissa) deterministically:
                feed_u8(h, u8::from(d.is_sign_negative()));
                feed_u32(h, d.scale());
                feed_bytes(h, &d.mantissa().to_be_bytes());
            }
            Self::E8s(v) => {
                feed_u64(h, v.get());
            }
            Self::E18s(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float32(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float64(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Int(i) => {
                feed_i64(h, *i);
            }
            Self::Principal(p) => {
                let raw = p.as_slice();
                feed_u32(h, raw.len() as u32);
                feed_bytes(h, raw);
            }
            Self::Subaccount(s) => {
                feed_bytes(h, &s.to_bytes());
            }
            Self::Text(s) => {
                // If you need case/Unicode insensitivity, normalize; else skip (much faster)
                // let norm = normalize_nfkc_casefold(s);
                // feed_u32( h, norm.len() as u32);
                // feed_bytes( h, norm.as_bytes());
                feed_u32(h, s.len() as u32);
                feed_bytes(h, s.as_bytes());
            }
            Self::Timestamp(t) => {
                feed_u64(h, t.get());
            }
            Self::Uint(u) => {
                feed_u64(h, *u);
            }
            Self::Ulid(u) => {
                feed_bytes(h, &u.to_bytes());
            }
            Self::List(xs) => {
                feed_u32(h, xs.len() as u32);
                for x in xs {
                    feed_u8(h, 0xFF);
                    x.write_to_hasher(h); // recurse, no sub-hash
                }
            }
            Self::None | Self::Unit | Self::Unsupported => {}
        }
    }

    #[must_use]
    pub fn hash_value(&self) -> [u8; 16] {
        const VERSION: u8 = 1;

        let mut h = Xxh3::with_seed(0);
        feed_u8(&mut h, VERSION); // version

        self.write_to_hasher(&mut h);
        h.digest128().to_be_bytes()
    }
}
