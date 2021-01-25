Documentation coming soon. Geared towards netcode, so all types are serialized in network-byte-order.

Why not just use serde? Serde has basic types, such as strings, which are variable-length and require some sort of format to serialize. This crate chooses to not force a particular protocol on the user, so if you have a variable-length type, you will need to implement a serialization protocol on it yourself.

Sample usage:

```rust
use std::io::{self, Cursor, Read, Write};

use netpack::{PackError, Packable, Unpackable, unpack, unpack_from};

// Sample struct with a sub-struct that also implements Packable/Unpackable
#[derive(Debug, PartialEq)]
struct Sample {
    num: i128,
    sample: SubSample,
    b: bool,
}

// Implement Packable for our struct
impl Packable for Sample {
    type Error = io::Error; // Error packables use by default

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        self.num.pack_into(stream)?;
        self.sample.pack_into(stream)?;
        self.b.pack_into(stream)?;
        Ok(())
    }
}

// Implement Unpackable for our struct
impl Unpackable for Sample {
    type Error = PackError; // Error unpackables use by default

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(Self {
            num: unpack_from(rdr)?,
            sample: unpack_from(rdr)?,
            b: unpack_from(rdr)?,
        })
    }
}

// Embedded structure that's also packable/unpackable
#[derive(Debug, PartialEq)]
struct SubSample {
    num: i32,
    arr: [u16; 5],
    float: f32,
}

impl Packable for SubSample {
    type Error = io::Error; // Error packables use by default

    fn pack_into(&self, stream: &mut impl Write) -> Result<(), Self::Error> {
        self.num.pack_into(stream)?;
        self.arr.pack_into(stream)?;
        self.float.pack_into(stream)?;
        Ok(())
    }
}

impl Unpackable for SubSample {
    type Error = PackError; // Error unpackables use by default

    fn unpack_from(rdr: &mut impl Read) -> Result<Self, Self::Error> {
        Ok(Self {
            num: unpack_from(rdr)?,
            arr: unpack_from(rdr)?,
            float: unpack_from(rdr)?,
        })
    }
}

fn main() {
    let sub = SubSample {
        num: 20,
        arr: [10, 100, 1000, 5, 0],
        float: 0.3
    };
    let sample = Sample {
        num: -5,
        sample: sub,
        b: true
    };

    let mut v = Vec::new();
    // Create a Write stream for the vec
    let mut c = Cursor::new(&mut v);

    // Serialize our struct into the stream
    sample.pack_into(&mut c).unwrap();
    // Serialized data is now in the vec
    assert_eq!(&v, &vec![
        // Sample::num
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 251,
            // SubSample::num
            0, 0, 0, 20,
            // SubSample::arr
            0, 10, 0, 100, 3, 232, 0, 5, 0, 0,
            // SubSample::float
            62, 153, 153, 154,
        // Sample::b
        1
    ]);

    // Can also return a vec
    let mut v = sample.pack().unwrap();
    // Serialized data is now in the vec
    assert_eq!(&v, &vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 251, 0, 0, 0, 20, 0, 10, 0, 100, 3, 232, 0, 5, 0, 0, 62, 153, 153, 154, 1]);

    // Extra data simulating a buffer with more than just our struct
    v.push(0xff);

    // Unpacking is just as simple:
    let mut c = Cursor::new(&mut v);
    let new_sample: Sample = unpack_from(&mut c).unwrap();
    assert_eq!(&new_sample, &sample);

    // Or from a slice:
    let (new_sample, rest): (Sample, &[u8]) = unpack(&v).unwrap();
    assert_eq!(&new_sample, &sample);
    // rest contains remaining bytes that weren't part of our struct
    assert_eq!(&rest, &[0xff]);
}
```