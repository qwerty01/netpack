use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{convert::TryInto, fmt::{self, Display, Formatter}};
use std::io::{self, Cursor, Read, Write};

pub trait Packable {
    type Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error>;
    fn pack(&self) -> Result<Vec<u8>, Self::Error> {
        let mut wtr = Vec::new();
        self.pack_into(&mut wtr)?;
        Ok(wtr)
    }
}
pub trait Unpackable where Self: Sized {
    type Error;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error>;
    fn unpack<'a>(buf: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let mut rdr = Cursor::new(buf);
        let val = Self::unpack_from(&mut rdr)?;
        Ok((val, &buf[rdr.position() as usize..]))
    }
}

#[derive(Debug, PartialEq)]
pub enum PackError {
    SizeError,
}
impl Display for PackError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SizeError => write!(f, "buffer is too small"),
        }
    }
}
impl std::error::Error for PackError {}
impl From<io::Error> for PackError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => Self::SizeError,
            _ => unreachable!(),
        }
    }
}

impl Packable for bool {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u8(if *self {1} else {0})?;
        Ok(())
    }
}
impl Unpackable for bool {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u8()? != 0)
    }
}

impl Packable for u8 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u8(*self)?;
        Ok(())
    }
}
impl Unpackable for u8 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u8()?)
    }
}

impl Packable for i8 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_i8(*self)?;
        Ok(())
    }
}
impl Unpackable for i8 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_i8()?)
    }
}

impl Packable for u16 {
    type Error = io::Error;
    
    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for u16 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u16::<BigEndian>()?)
    }
}

impl Packable for i16 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_i16::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for i16 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_i16::<BigEndian>()?)
    }
}

impl Packable for u32 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for u32 {
    type Error = PackError;
    
    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u32::<BigEndian>()?)
    }
}

impl Packable for i32 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for i32 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_i32::<BigEndian>()?)
    }
}

impl Packable for u64 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for u64 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u64::<BigEndian>()?)
    }
}

impl Packable for i64 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for i64 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_i64::<BigEndian>()?)
    }
}

impl Packable for u128 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_u128::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for u128 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_u128::<BigEndian>()?)
    }
}

impl Packable for i128 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_i128::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for i128 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_i128::<BigEndian>()?)
    }
}

impl Packable for f32 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_f32::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for f32 {
    type Error = PackError;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_f32::<BigEndian>()?)
    }
}

impl Packable for f64 {
    type Error = io::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        stream.write_f64::<BigEndian>(*self)?;
        Ok(())
    }
}
impl Unpackable for f64 {
    type Error = PackError;
    
    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(rdr.read_f64::<BigEndian>()?)
    }
}

impl<T: Packable, const S: usize> Packable for [T; S] {
    type Error = T::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        for v in self {
            v.pack_into(stream)?;
        }

        Ok(())
    }
}
impl<T: Unpackable, const S: usize> Unpackable for [T; S] {
    type Error = T::Error;

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        let mut v: Vec<T> = Vec::with_capacity(S);

        for _ in 0..S {
            v.push(unpack_from(rdr)?);
        }

        Ok(v.try_into().unwrap_or_else(|_| unreachable!()))
    }
}

impl<T: Packable> Packable for Vec<T> {
    type Error = T::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        for v in self {
            v.pack_into(stream)?;
        }

        Ok(())
    }
}

impl<T: Packable> Packable for &T {
    type Error = T::Error;

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        (*self).pack_into(stream)
    }
}

pub fn unpack_from<T: Unpackable>(rdr: &mut impl Read) -> Result<T, T::Error> {
    T::unpack_from(rdr)
}
pub fn unpack<'a, T: Unpackable>(buf: &'a [u8]) -> Result<(T, &'a [u8]), T::Error> {
    T::unpack(buf)
}

pub fn pack_into<T: Packable>(wtr: &mut impl Write, value: &T) -> Result<(), T::Error> {
    T::pack_into(value, wtr)
}
pub fn pack<T: Packable>(value: &T) -> Result<Vec<u8>, T::Error> {
    T::pack(value)
}

pub trait WritePackExt: io::Write + Sized {
    fn pack<T: Packable>(&mut self, pack: &T) -> Result<(), T::Error>;
}
impl<T: Write> WritePackExt for T {
    fn pack<P: Packable>(&mut self, pack: &P) -> Result<(), P::Error> {
        pack.pack_into(self)?;
        Ok(())
    }
}

pub trait ReadPackExt: io::Read {
    fn unpack<T: Unpackable>(&mut self) -> Result<T, T::Error>;
}
impl<T: Read> ReadPackExt for T {
    fn unpack<U: Unpackable>(&mut self) -> Result<U, U::Error> {
        U::unpack_from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_pack_bool() {
        let t = true;
        let f = false;
        let tb = t.pack().unwrap();
        let fb = f.pack().unwrap();

        assert_eq!(tb, vec![0x01]);
        assert_eq!(fb, vec![0x00]);
    }

    #[test]
    fn test_unpack_bool() {
        let tb1 = [];
        let tb2 = [0x1];
        let tb3 = [0x1, 0xff];
        let tb4 = [0x20];
        let tb5 = [0x20, 0xff];
        let fb6 = [0];
        let fb7 = [0, 0xff];

        let tv1: Result<(i8, _), PackError> = unpack(&tb1);
        let (tv2, tv2b): (bool, _) = unpack(&tb2).unwrap();
        let (tv3, tv3b): (bool, _) = unpack(&tb3).unwrap();
        let (tv4, tv4b): (bool, _) = unpack(&tb4).unwrap();
        let (tv5, tv5b): (bool, _) = unpack(&tb5).unwrap();
        let (fv6, fvb6): (bool, _) = unpack(&fb6).unwrap();
        let (fv7, fvb7): (bool, _) = unpack(&fb7).unwrap();

        assert_eq!(tv1, Err(PackError::SizeError));
        assert_eq!(tv2, true);
        assert_eq!(tv2b, &[]);
        assert_eq!(tv3, true);
        assert_eq!(tv3b, &[0xff]);
        assert_eq!(tv4, true);
        assert_eq!(tv4b, &[]);
        assert_eq!(tv5, true);
        assert_eq!(tv5b, &[0xff]);
        assert_eq!(fv6, false);
        assert_eq!(fvb6, &[]);
        assert_eq!(fv7, false);
        assert_eq!(fvb7, &[0xff]);
    }
    
    #[test]
    fn test_roundtrip_bool() {
        let v1 = true;
        let v2 = false;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, t1b): (bool, _) = unpack(&i1).unwrap();
        let (t2, t2b): (bool, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(t1b, &[]);
        assert_eq!(t2, v2);
        assert_eq!(t2b, &[]);
    }

    #[test]
    fn test_pack_u8() {
        let val = 0xfu8;

        assert_eq!(val.pack().unwrap(), vec![val]);
    }
    
    #[test]
    fn test_unpack_u8() {
        let u1 = [];
        let u2 = [0x82];
        let u3 = [0x82, 0xff];

        let v1: Result<(i8, _), PackError> = unpack(&u1);
        let (v2, b2): (u8, _) = unpack(&u2).unwrap();
        let (v3, b3): (u8, _) = unpack(&u3).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, 0x82);
        assert_eq!(b2, &[]);
        assert_eq!(v3, 0x82);
        assert_eq!(b3, &[0xff]);
    }

    #[test]
    fn test_roundtrip_u8() {
        let v = 0x20;

        let i = v.pack().unwrap();

        let (t, tb): (u8, _) = unpack(&i).unwrap();

        assert_eq!(t, v);
        assert_eq!(tb, &[]);
    }

    #[test]
    fn test_pack_i8() {
        let val = 25i8;
        let val2 = -25i8;

        assert_eq!(val.pack().unwrap(), vec![25]);
        assert_eq!(val2.pack().unwrap(), vec![231]);
    }

    #[test]
    fn test_unpack_i8() {
        let i1 = [];
        let i2 = [25];
        let i3 = [25, 0xff];
        let i4 = [231];
        let i5 = [231, 0xff];

        let v1: Result<(i8, _), PackError> = unpack(&i1);
        let (v2, b2): (i8, _) = unpack(&i2).unwrap();
        let (v3, b3): (i8, _) = unpack(&i3).unwrap();
        let (v4, b4): (i8, _) = unpack(&i4).unwrap();
        let (v5, b5): (i8, _) = unpack(&i5).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, 25);
        assert_eq!(b2, &[]);
        assert_eq!(v3, 25);
        assert_eq!(b3, &[0xff]);
        assert_eq!(v4, -25);
        assert_eq!(b4, &[]);
        assert_eq!(v5, -25);
        assert_eq!(b5, &[0xff]);
    }

    #[test]
    fn test_roundtrip_i8() {
        let v1 = 25;
        let v2 = -25;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, t1b): (i8, _) = unpack(&i1).unwrap();
        let (t2, t2b): (i8, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(t1b, &[]);
        assert_eq!(t2, v2);
        assert_eq!(t2b, &[]);
    }

    #[test]
    fn test_pack_u16() {
        let v1 = 0x1221u16;

        assert_eq!(v1.pack().unwrap(), vec![0x12, 0x21]);
    }

    #[test]
    fn test_unpack_u16() {
        let i1 = [];
        let i2 = [0x12];
        let i3 = [0x12, 0x21];
        let i4 = [0x12, 0x21, 0xff];

        let v1: Result<(u16, _), PackError> = unpack(&i1);
        let v2: Result<(u16, _), PackError> = unpack(&i2);
        let (v3, b3): (u16, _) = unpack(&i3).unwrap();
        let (v4, b4): (u16, _) = unpack(&i4).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, 0x1221);
        assert_eq!(b3, &[]);
        assert_eq!(v4, 0x1221);
        assert_eq!(b4, &[0xff]);
    }

    #[test]
    fn test_roundtrip_u16() {
        let v1 = 0x1221u16;

        let i1 = v1.pack().unwrap();
        
        let (t1, t1b): (u16, _) = unpack(&i1).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(t1b, &[]);
    }

    #[test]
    fn test_pack_i16() {
        let v1 = 0x1221i16;
        let v2 = -0x1221i16;

        assert_eq!(v1.pack().unwrap(), vec![0x12, 0x21]);
        assert_eq!(v2.pack().unwrap(), vec![0xed, 0xdf]);
    }
    
    #[test]
    fn test_unpack_i16() {
        let i1 = [0x12];
        let i2 = [0x12, 0x21];
        let i3 = [0x12, 0x21, 0xff];
        let i4 = [0xed, 0xdf];
        let i5 = [0xed, 0xdf, 0xff];

        let v1: Result<(i16, _), PackError> = unpack(&i1);
        let (v2, b2): (i16, _) = unpack(&i2).unwrap();
        let (v3, b3): (i16, _) = unpack(&i3).unwrap();
        let (v4, b4): (i16, _) = unpack(&i4).unwrap();
        let (v5, b5): (i16, _) = unpack(&i5).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, 0x1221);
        assert_eq!(b2, &[]);
        assert_eq!(v3, 0x1221);
        assert_eq!(b3, &[0xff]);
        assert_eq!(v4, -0x1221);
        assert_eq!(b4, &[]);
        assert_eq!(v5, -0x1221);
        assert_eq!(b5, &[0xff]);
    }

    #[test]
    fn test_roundtrip_i16() {
        let v1 = 0x1221;
        let v2 = -0x1221;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (i16, _) = unpack(&i1).unwrap();
        let (t2, b2): (i16, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_u32() {
        let v1 = 0x12345678u32;

        assert_eq!(v1.pack().unwrap(), vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_unpack_u32() {
        let i1 = [];
        let i2 = [0x12];
        let i3 = [0x12, 0x34];
        let i4 = [0x12, 0x34, 0x56];
        let i5 = [0x12, 0x34, 0x56, 0x78];
        let i6 = [0x12, 0x34, 0x56, 0x78, 0xff];

        let v1: Result<(u32, _), PackError> = unpack(&i1);
        let v2: Result<(u32, _), PackError> = unpack(&i2);
        let v3: Result<(u32, _), PackError> = unpack(&i3);
        let v4: Result<(u32, _), PackError> = unpack(&i4);
        let (v5, b5): (u32, _) = unpack(&i5).unwrap();
        let (v6, b6): (u32, _) = unpack(&i6).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, 0x12345678);
        assert_eq!(b5, &[]);
        assert_eq!(v6, 0x12345678);
        assert_eq!(b6, &[0xff]);
    }

    #[test]
    fn test_roundtrip_u32() {
        let v = 0x12345678u32;

        let i = v.pack().unwrap();

        let (t1, b1): (u32, _) = unpack(&i).unwrap();

        assert_eq!(t1, v);
        assert_eq!(b1, &[]);
    }

    #[test]
    fn test_pack_i32() {
        let v1 = 0x12345678i32;
        let v2 = -0x12345678i32;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        assert_eq!(i1, vec![0x12, 0x34, 0x56, 0x78]);
        assert_eq!(i2, vec![0xed, 0xcb, 0xa9, 0x88]);
    }

    #[test]
    fn test_unpack_i32() {
        let i1 = [];
        let i2 = [0x12];
        let i3 = [0x12, 0x34];
        let i4 = [0x12, 0x34, 0x56];
        let i5 = [0x12, 0x34, 0x56, 0x78];
        let i6 = [0x12, 0x34, 0x56, 0x78, 0xff];
        let i7 = [0xed, 0xcb, 0xa9, 0x88];
        let i8 = [0xed, 0xcb, 0xa9, 0x88, 0xff];

        let v1: Result<(i32, _), PackError> = unpack(&i1);
        let v2: Result<(i32, _), PackError> = unpack(&i2);
        let v3: Result<(i32, _), PackError> = unpack(&i3);
        let v4: Result<(i32, _), PackError> = unpack(&i4);
        let (v5, b5): (i32, _) = unpack(&i5).unwrap();
        let (v6, b6): (i32, _) = unpack(&i6).unwrap();
        let (v7, b7): (i32, _) = unpack(&i7).unwrap();
        let (v8, b8): (i32, _) = unpack(&i8).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, 0x12345678);
        assert_eq!(b5, &[]);
        assert_eq!(v6, 0x12345678);
        assert_eq!(b6, &[0xff]);
        assert_eq!(v7, -0x12345678);
        assert_eq!(b7, &[]);
        assert_eq!(v8, -0x12345678);
        assert_eq!(b8, &[0xff]);
    }

    #[test]
    fn test_roundtrip_i32() {
        let v1 = 0x12345678;
        let v2 = -0x12345678;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (i32, _) = unpack(&i1).unwrap();
        let (t2, b2): (i32, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_u64() {
        let v1 = 0x0123456789abcdefu64;
        
        assert_eq!(v1.pack().unwrap(), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    }

    #[test]
    fn test_unpack_u64() {
        let i1 = [];
        let i2 = [0x01];
        let i3 = [0x01, 0x23];
        let i4 = [0x01, 0x23, 0x45];
        let i5 = [0x01, 0x23, 0x45, 0x67];
        let i6 = [0x01, 0x23, 0x45, 0x67, 0x89];
        let i7 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let i8 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd];
        let i9 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let i10 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff];

        let v1: Result<(u64, _), PackError> = unpack(&i1);
        let v2: Result<(u64, _), PackError> = unpack(&i2);
        let v3: Result<(u64, _), PackError> = unpack(&i3);
        let v4: Result<(u64, _), PackError> = unpack(&i4);
        let v5: Result<(u64, _), PackError> = unpack(&i5);
        let v6: Result<(u64, _), PackError> = unpack(&i6);
        let v7: Result<(u64, _), PackError> = unpack(&i7);
        let v8: Result<(u64, _), PackError> = unpack(&i8);
        let (v9, b9): (u64, _) = unpack(&i9).unwrap();
        let (v10, b10): (u64, _) = unpack(&i10).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, Err(PackError::SizeError));
        assert_eq!(v6, Err(PackError::SizeError));
        assert_eq!(v7, Err(PackError::SizeError));
        assert_eq!(v8, Err(PackError::SizeError));
        assert_eq!(v9, 0x0123456789abcdef);
        assert_eq!(b9, &[]);
        assert_eq!(v10, 0x0123456789abcdef);
        assert_eq!(b10, &[0xff]);
    }

    #[test]
    fn test_roundtrip_u64() {
        let v = 0x0123456789abcdefu64;

        let i = v.pack().unwrap();

        let (t1, b1): (u64, _) = unpack(&i).unwrap();

        assert_eq!(t1, v);
        assert_eq!(b1, &[]);
    } 

    #[test]
    fn test_pack_i64() {
        let v1 = 0x123456789abcdefi64;
        let v2 = -0x123456789abcdefi64;

        assert_eq!(v1.pack().unwrap(), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        assert_eq!(v2.pack().unwrap(), vec![0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11]);
    }

    #[test]
    fn test_unpack_i64() {
        let i1 = [];
        let i2 = [0x01];
        let i3 = [0x01, 0x23];
        let i4 = [0x01, 0x23, 0x45];
        let i5 = [0x01, 0x23, 0x45, 0x67];
        let i6 = [0x01, 0x23, 0x45, 0x67, 0x89];
        let i7 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let i8 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd];
        let i9 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let i10 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff];
        let i11 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11];
        let i12 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11, 0xff];

        let v1: Result<(i64, _), PackError> = unpack(&i1);
        let v2: Result<(i64, _), PackError> = unpack(&i2);
        let v3: Result<(i64, _), PackError> = unpack(&i3);
        let v4: Result<(i64, _), PackError> = unpack(&i4);
        let v5: Result<(i64, _), PackError> = unpack(&i5);
        let v6: Result<(i64, _), PackError> = unpack(&i6);
        let v7: Result<(i64, _), PackError> = unpack(&i7);
        let v8: Result<(i64, _), PackError> = unpack(&i8);
        let (v9, b9): (i64, _) = unpack(&i9).unwrap();
        let (v10, b10): (i64, _) = unpack(&i10).unwrap();
        let (v11, b11): (i64, _) = unpack(&i11).unwrap();
        let (v12, b12): (i64, _) = unpack(&i12).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, Err(PackError::SizeError));
        assert_eq!(v6, Err(PackError::SizeError));
        assert_eq!(v7, Err(PackError::SizeError));
        assert_eq!(v8, Err(PackError::SizeError));
        assert_eq!(v9, 0x0123456789abcdef);
        assert_eq!(b9, &[]);
        assert_eq!(v10, 0x0123456789abcdef);
        assert_eq!(b10, &[0xff]);
        assert_eq!(v11, -0x0123456789abcdef);
        assert_eq!(b11, &[]);
        assert_eq!(v12, -0x0123456789abcdef);
        assert_eq!(b12, &[0xff]);
    }

    #[test]
    fn test_roundtrip_i64() {
        let v1 = 0x123456789abcdefi64;
        let v2 = -0x123456789abcdefi64;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (i64, _) = unpack(&i1).unwrap();
        let (t2, b2): (i64, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_u128() {
        let v1 = 0x0123456789abcdeffedcba9876543210u128;

        assert_eq!(v1.pack().unwrap(), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]);
    }

    #[test]
    fn test_unpack_u128() {
        let i1 = [];
        let i2 = [0x01];
        let i3 = [0x01, 0x23];
        let i4 = [0x01, 0x23, 0x45];
        let i5 = [0x01, 0x23, 0x45, 0x67];
        let i6 = [0x01, 0x23, 0x45, 0x67, 0x89];
        let i7 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let i8 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd];
        let i9 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let i10 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe];
        let i11 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc];
        let i12 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba];
        let i13 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98];
        let i14 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76];
        let i15 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54];
        let i16 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32];
        let i17 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10];
        let i18 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xff];

        let v1: Result<(u128, _), PackError> = unpack(&i1);
        let v2: Result<(u128, _), PackError> = unpack(&i2);
        let v3: Result<(u128, _), PackError> = unpack(&i3);
        let v4: Result<(u128, _), PackError> = unpack(&i4);
        let v5: Result<(u128, _), PackError> = unpack(&i5);
        let v6: Result<(u128, _), PackError> = unpack(&i6);
        let v7: Result<(u128, _), PackError> = unpack(&i7);
        let v8: Result<(u128, _), PackError> = unpack(&i8);
        let v9: Result<(u128, _), PackError> = unpack(&i9);
        let v10: Result<(u128, _), PackError> = unpack(&i10);
        let v11: Result<(u128, _), PackError> = unpack(&i11);
        let v12: Result<(u128, _), PackError> = unpack(&i12);
        let v13: Result<(u128, _), PackError> = unpack(&i13);
        let v14: Result<(u128, _), PackError> = unpack(&i14);
        let v15: Result<(u128, _), PackError> = unpack(&i15);
        let v16: Result<(u128, _), PackError> = unpack(&i16);
        let (v17, b17): (u128, _) = unpack(&i17).unwrap();
        let (v18, b18): (u128, _) = unpack(&i18).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, Err(PackError::SizeError));
        assert_eq!(v6, Err(PackError::SizeError));
        assert_eq!(v7, Err(PackError::SizeError));
        assert_eq!(v8, Err(PackError::SizeError));
        assert_eq!(v9, Err(PackError::SizeError));
        assert_eq!(v10, Err(PackError::SizeError));
        assert_eq!(v11, Err(PackError::SizeError));
        assert_eq!(v12, Err(PackError::SizeError));
        assert_eq!(v13, Err(PackError::SizeError));
        assert_eq!(v14, Err(PackError::SizeError));
        assert_eq!(v15, Err(PackError::SizeError));
        assert_eq!(v16, Err(PackError::SizeError));
        assert_eq!(v17, 0x0123456789abcdeffedcba9876543210);
        assert_eq!(b17, &[]);
        assert_eq!(v18, 0x0123456789abcdeffedcba9876543210);
        assert_eq!(b18, &[0xff]);
    }

    #[test]
    fn test_roundtrip_u128() {
        let v = 0x0123456789abcdeffedcba9876543210u128;

        let i = v.pack().unwrap();

        let (t1, b1): (u128, _) = unpack(&i).unwrap();

        assert_eq!(t1, v);
        assert_eq!(b1, &[]);
    }

    #[test]
    fn test_pack_i128() {
        let v1 = 0x0123456789abcdeffedcba9876543210i128;
        let v2 = -0x0123456789abcdeffedcba9876543210i128;

        assert_eq!(v1.pack().unwrap(), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]);
        assert_eq!(v2.pack().unwrap(), vec![0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xf0]);
    }

    #[test]
    fn test_unpack_i128() {
        let i1 = [];
        let i2 = [0x01];
        let i3 = [0x01, 0x23];
        let i4 = [0x01, 0x23, 0x45];
        let i5 = [0x01, 0x23, 0x45, 0x67];
        let i6 = [0x01, 0x23, 0x45, 0x67, 0x89];
        let i7 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let i8 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd];
        let i9 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let i10 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe];
        let i11 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc];
        let i12 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba];
        let i13 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98];
        let i14 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76];
        let i15 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54];
        let i16 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32];
        let i17 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10];
        let i18 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xff];
        let i19 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xf0];
        let i20 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xf0, 0xff];

        let v1: Result<(i128, _), PackError> = unpack(&i1);
        let v2: Result<(i128, _), PackError> = unpack(&i2);
        let v3: Result<(i128, _), PackError> = unpack(&i3);
        let v4: Result<(i128, _), PackError> = unpack(&i4);
        let v5: Result<(i128, _), PackError> = unpack(&i5);
        let v6: Result<(i128, _), PackError> = unpack(&i6);
        let v7: Result<(i128, _), PackError> = unpack(&i7);
        let v8: Result<(i128, _), PackError> = unpack(&i8);
        let v9: Result<(i128, _), PackError> = unpack(&i9);
        let v10: Result<(i128, _), PackError> = unpack(&i10);
        let v11: Result<(i128, _), PackError> = unpack(&i11);
        let v12: Result<(i128, _), PackError> = unpack(&i12);
        let v13: Result<(i128, _), PackError> = unpack(&i13);
        let v14: Result<(i128, _), PackError> = unpack(&i14);
        let v15: Result<(i128, _), PackError> = unpack(&i15);
        let v16: Result<(i128, _), PackError> = unpack(&i16);
        let (v17, b17): (i128, _) = unpack(&i17).unwrap();
        let (v18, b18): (i128, _) = unpack(&i18).unwrap();
        let (v19, b19): (i128, _) = unpack(&i19).unwrap();
        let (v20, b20): (i128, _) = unpack(&i20).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, Err(PackError::SizeError));
        assert_eq!(v6, Err(PackError::SizeError));
        assert_eq!(v7, Err(PackError::SizeError));
        assert_eq!(v8, Err(PackError::SizeError));
        assert_eq!(v9, Err(PackError::SizeError));
        assert_eq!(v10, Err(PackError::SizeError));
        assert_eq!(v11, Err(PackError::SizeError));
        assert_eq!(v12, Err(PackError::SizeError));
        assert_eq!(v13, Err(PackError::SizeError));
        assert_eq!(v14, Err(PackError::SizeError));
        assert_eq!(v15, Err(PackError::SizeError));
        assert_eq!(v16, Err(PackError::SizeError));
        assert_eq!(v17, 0x0123456789abcdeffedcba9876543210);
        assert_eq!(b17, &[]);
        assert_eq!(v18, 0x0123456789abcdeffedcba9876543210);
        assert_eq!(b18, &[0xff]);
        assert_eq!(v19, -0x0123456789abcdeffedcba9876543210);
        assert_eq!(b19, &[]);
        assert_eq!(v20, -0x0123456789abcdeffedcba9876543210);
        assert_eq!(b20, &[0xff]);
    }

    #[test]
    fn test_roundtrip_i128() {
        let v1 = 0x0123456789abcdeffedcba9876543210i128;
        let v2 = -0x0123456789abcdeffedcba9876543210i128;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (i128, _) = unpack(&i1).unwrap();
        let (t2, b2): (i128, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_f32() {
        let v1 = 0.00000000000000000000000000056904566f32;
        let v2 = -7878801000000000000000000000.0f32;

        assert_eq!(v1.pack().unwrap(), &[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(v2.pack().unwrap(), &[0xed, 0xcb, 0xa9, 0x88]);
    }
    
    #[test]
    fn test_unpack_f32() {
        let i1 = [];
        let i2 = [0x12];
        let i3 = [0x12, 0x34];
        let i4 = [0x12, 0x34, 0x56];
        let i5 = [0x12, 0x34, 0x56, 0x78];
        let i6 = [0x12, 0x34, 0x56, 0x78, 0xff];
        let i7 = [0xed, 0xcb, 0xa9, 0x88];
        let i8 = [0xed, 0xcb, 0xa9, 0x88, 0xff];

        let v1: Result<(f32, _), PackError> = unpack(&i1);
        let v2: Result<(f32, _), PackError> = unpack(&i2);
        let v3: Result<(f32, _), PackError> = unpack(&i3);
        let v4: Result<(f32, _), PackError> = unpack(&i4);
        let (v5, b5): (f32, _) = unpack(&i5).unwrap();
        let (v6, b6): (f32, _) = unpack(&i6).unwrap();
        let (v7, b7): (f32, _) = unpack(&i7).unwrap();
        let (v8, b8): (f32, _) = unpack(&i8).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, 0.00000000000000000000000000056904566);
        assert_eq!(b5, &[]);
        assert_eq!(v6, 0.00000000000000000000000000056904566);
        assert_eq!(b6, &[0xff]);
        assert_eq!(v7, -7878801000000000000000000000.0);
        assert_eq!(b7, &[]);
        assert_eq!(v8, -7878801000000000000000000000.0);
        assert_eq!(b8, &[0xff]);
    }

    #[test]
    fn test_roundtrip_f32() {
        let v1 = 0.00000000000000000000000000056904566f32;
        let v2 = -7878801000000000000000000000.0f32;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (f32, _) = unpack(&i1).unwrap();
        let (t2, b2): (f32, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_f64() {
        let v1 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003512700564088504;
        let v2 = -1231330068773694700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

        assert_eq!(v1.pack().unwrap(), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        assert_eq!(v2.pack().unwrap(), vec![0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11]);
    }

    #[test]
    fn test_unpack_f64() {
        let i1 = [];
        let i2 = [0x01];
        let i3 = [0x01, 0x23];
        let i4 = [0x01, 0x23, 0x45];
        let i5 = [0x01, 0x23, 0x45, 0x67];
        let i6 = [0x01, 0x23, 0x45, 0x67, 0x89];
        let i7 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let i8 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd];
        let i9 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let i10 = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff];
        let i11 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11];
        let i12 = [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x11, 0xff];

        let v1: Result<(f64, _), PackError> = unpack(&i1);
        let v2: Result<(f64, _), PackError> = unpack(&i2);
        let v3: Result<(f64, _), PackError> = unpack(&i3);
        let v4: Result<(f64, _), PackError> = unpack(&i4);
        let v5: Result<(f64, _), PackError> = unpack(&i5);
        let v6: Result<(f64, _), PackError> = unpack(&i6);
        let v7: Result<(f64, _), PackError> = unpack(&i7);
        let v8: Result<(f64, _), PackError> = unpack(&i8);
        let (v9, b9): (f64, _) = unpack(&i9).unwrap();
        let (v10, b10): (f64, _) = unpack(&i10).unwrap();
        let (v11, b11): (f64, _) = unpack(&i11).unwrap();
        let (v12, b12): (f64, _) = unpack(&i12).unwrap();

        assert_eq!(v1, Err(PackError::SizeError));
        assert_eq!(v2, Err(PackError::SizeError));
        assert_eq!(v3, Err(PackError::SizeError));
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, Err(PackError::SizeError));
        assert_eq!(v6, Err(PackError::SizeError));
        assert_eq!(v7, Err(PackError::SizeError));
        assert_eq!(v8, Err(PackError::SizeError));
        assert_eq!(v9, 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003512700564088504);
        assert_eq!(b9, &[]);
        assert_eq!(v10, 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003512700564088504);
        assert_eq!(b10, &[0xff]);
        assert_eq!(v11, -1231330068773694700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0);
        assert_eq!(b11, &[]);
        assert_eq!(v12, -1231330068773694700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0);
        assert_eq!(b12, &[0xff]);
    }

    #[test]
    fn test_roundtrip_f64() {
        let v1 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003512700564088504;
        let v2 = -1231330068773694700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

        let i1 = v1.pack().unwrap();
        let i2 = v2.pack().unwrap();

        let (t1, b1): (f64, _) = unpack(&i1).unwrap();
        let (t2, b2): (f64, _) = unpack(&i2).unwrap();

        assert_eq!(t1, v1);
        assert_eq!(b1, &[]);
        assert_eq!(t2, v2);
        assert_eq!(b2, &[]);
    }

    #[test]
    fn test_pack_array() {
        let a1: [u8; 0] = [];
        let a2: [u8; 1] = [0x01];
        let a3: [u8; 2] = [0x01, 0x23];
        let a4: [u16; 3] = [0x01, 0x23, 0x45];

        assert_eq!(a1.pack().unwrap(), &a1);
        assert_eq!(a2.pack().unwrap(), &a2);
        assert_eq!(a3.pack().unwrap(), &a3);
        assert_eq!(a4.pack().unwrap(), &[0x00, 0x01, 0x00, 0x23, 0x00, 0x45]);
    }

    #[test]
    fn test_unpack_array() {
        let a1 = [];
        let a2 = [0x01];
        let a3 = [0x01, 0x23];
        let a4 = [0x00, 0x01, 0x00, 0x23, 0x00];
        let a5 = [0x00, 0x01, 0x00, 0x23, 0x00, 0x45];
        let a6 = [0x00, 0x01, 0x00, 0x23, 0x00, 0x45, 0xff];
        let a7 = [0x01, 0xff];

        let (v1, b1): ([u8; 0], _) = unpack(&a1).unwrap();
        let (v2, b2): ([u8; 1], _) = unpack(&a2).unwrap();
        let (v3, b3): ([u8; 2], _) = unpack(&a3).unwrap();
        let v4: Result<([u16; 3], _), PackError> = unpack(&a4);
        let (v5, b5): ([u16; 3], _) = unpack(&a5).unwrap();
        let (v6, b6): ([u16; 3], _) = unpack(&a6).unwrap();
        let (v7, b7): ([u8; 1], _) = unpack(&a7).unwrap();

        assert_eq!(v1, []);
        assert_eq!(b1, &[]);
        assert_eq!(v2, [0x01]);
        assert_eq!(b2, &[]);
        assert_eq!(v3, [0x01, 0x23]);
        assert_eq!(b3, &[]);
        assert_eq!(v4, Err(PackError::SizeError));
        assert_eq!(v5, [0x01, 0x23, 0x45]);
        assert_eq!(b5, &[]);
        assert_eq!(v6, [0x01, 0x23, 0x45]);
        assert_eq!(b6, &[0xff]);
        assert_eq!(v7, [0x01]);
        assert_eq!(b7, &[0xff]);
    }

    #[test]
    fn test_pack_vec() {
        let v1: Vec<u8> = vec![0x01, 0x23, 0x45];
        let v2: Vec<u16> = vec![0x0123, 0x4567];

        assert_eq!(v1.pack().unwrap(), [0x01, 0x23, 0x45]);
        assert_eq!(v2.pack().unwrap(), [0x01, 0x23, 0x45, 0x67]);
    }
}
