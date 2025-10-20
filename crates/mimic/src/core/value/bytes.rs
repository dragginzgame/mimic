use crate::core::value::Value;
use canic::utils::hash::Xxh3;

///
/// Canonical Byte Representation
///

#[inline]
fn feed_i32(h: &mut Xxh3, x: i32) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_i64(h: &mut Xxh3, x: i64) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_i128(h: &mut Xxh3, x: i128) {
    h.update(&x.to_be_bytes());
}
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
fn feed_u128(h: &mut Xxh3, x: u128) {
    h.update(&x.to_be_bytes());
}

#[inline]
fn feed_bytes(h: &mut Xxh3, b: &[u8]) {
    h.update(b);
}

#[allow(clippy::cast_possible_truncation)]
impl Value {
    ///
    /// Compute a **canonical, deterministic 128-bit fingerprint** of this `Value`.
    ///
    /// This is *not* the same as serializing the value (e.g. with CBOR or Serde) and hashing:
    /// - CBOR is not canonical by default (ints can have multiple encodings, maps can reorder keys, NaN payloads differ, etc.).
    /// - Rust's internal layout is not stable across versions or platforms.
    ///
    /// Instead, we define our own **canonical byte representation**:
    /// - Prefix with a fixed `VERSION` byte to allow evolution of the format.
    /// - Prefix with a `ValueTag` to distinguish enum variants (`Int(1)` vs `Uint(1)`).
    /// - Encode each variant deterministically (e.g. Decimal as sign/scale/mantissa).
    /// - Recurse through lists element-by-element in order.
    ///
    /// ### Why?
    /// - **Stable across upgrades / canisters**: the same logical value always yields the same hash.
    /// - **Indexing**: provides a fixed-size `[u8; 16]` fingerprint for use in secondary indexes
    ///   and fast equality lookups.
    /// - **Canonicalization**: ensures semantically equal values hash identically, avoiding
    ///   “same value, different bytes” bugs.
    ///
    /// Use this in query planning, index scans, and anywhere you need a compact,
    /// reproducible identity for a `Value`.
    ///
    fn write_to_hasher(&self, h: &mut Xxh3) {
        feed_u8(h, self.tag());

        match self {
            Self::Account(a) => {
                feed_bytes(h, &a.to_bytes());
            }
            Self::Blob(v) => {
                feed_u8(h, 0x01);
                feed_bytes(h, v);
            }
            Self::Bool(b) => {
                feed_u8(h, u8::from(*b));
            }
            Self::Date(d) => feed_i32(h, d.get()),
            Self::Decimal(d) => {
                // encode (sign, scale, mantissa) deterministically:
                feed_u8(h, u8::from(d.is_sign_negative()));
                feed_u32(h, d.scale());
                feed_bytes(h, &d.mantissa().to_be_bytes());
            }
            Self::Duration(t) => {
                feed_u64(h, t.get());
            }
            Self::Enum(v) => {
                feed_u32(h, v.path.len() as u32);
                feed_bytes(h, v.path.as_bytes());

                feed_u32(h, v.variant.len() as u32);
                feed_bytes(h, v.variant.as_bytes());
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
            Self::Int128(i) => {
                feed_i128(h, i.get());
            }
            Self::IntBig(v) => {
                feed_bytes(h, &v.to_leb128());
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
            Self::Uint128(u) => {
                feed_u128(h, u.get());
            }
            Self::UintBig(v) => {
                feed_bytes(h, &v.to_leb128());
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
