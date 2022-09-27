use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum List<'a> {
    Byte(Cow<'a, [i8]>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Cow<'a, [u8]>>),
    String(Vec<Cow<'a, str>>),
    List(Vec<List<'a>>),
    Compound(Vec<Compound<'a>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
    Invalid,
}

#[cfg(feature = "to_static")]
impl<'a> ToStatic for List<'a> {
    type Static = List<'static>;
    fn to_static(&self) -> Self::Static {
        match self {
            Self::Byte(bytes) => List::Byte(bytes.to_static()),
            Self::Short(shorts) => List::Short(shorts.to_static()),
            Self::Int(ints) => List::Int(ints.to_static()),
            Self::Long(longs) => List::Long(longs.to_static()),
            Self::Float(floats) => List::Float(floats.to_static()),
            Self::Double(doubles) => List::Double(doubles.to_static()),
            Self::ByteArray(bytearrays) => List::ByteArray(bytearrays.to_static()),
            Self::String(strings) => List::String(strings.to_static()),
            Self::List(lists) => List::List(lists.to_static()),
            Self::Compound(compounds) => List::Compound(compounds.to_static()),
            Self::IntArray(intarrays) => List::IntArray(intarrays.to_static()),
            Self::LongArray(longarrays) => List::LongArray(longarrays.to_static()),
            Self::Invalid => List::Invalid,
        }
    }
    fn into_static(self) -> Self::Static {
        match self {
            Self::Byte(bytes) => List::Byte(bytes.into_static()),
            Self::Short(shorts) => List::Short(shorts.into_static()),
            Self::Int(ints) => List::Int(ints.into_static()),
            Self::Long(longs) => List::Long(longs.into_static()),
            Self::Float(floats) => List::Float(floats.into_static()),
            Self::Double(doubles) => List::Double(doubles.into_static()),
            Self::ByteArray(bytearrays) => List::ByteArray(bytearrays.into_static()),
            Self::String(strings) => List::String(strings.into_static()),
            Self::List(lists) => List::List(lists.into_static()),
            Self::Compound(compounds) => List::Compound(compounds.into_static()),
            Self::IntArray(intarrays) => List::IntArray(intarrays.into_static()),
            Self::LongArray(longarrays) => List::LongArray(longarrays.into_static()),
            Self::Invalid => List::Invalid,
        }
    }
}

impl<'a> Encode for List<'a> {
    fn encode(&self, writer: &mut impl std::io::Write) -> miners_encoding::encode::Result<()> {
        match self {
            Self::Byte(bytes) => {
                NbtTag::Byte.encode(writer)?;
                <&Counted<_, i32>>::from(unsafe { &*(&bytes[..] as *const [i8] as *const [u8]) })
                    .encode(writer)
            }
            Self::Short(shorts) => {
                NbtTag::Short.encode(writer)?;
                <&Counted<_, i32>>::from(shorts).encode(writer)
            }
            Self::Int(ints) => {
                NbtTag::Int.encode(writer)?;
                <&Counted<_, i32>>::from(ints).encode(writer)
            }
            Self::Long(longs) => {
                NbtTag::Long.encode(writer)?;
                <&Counted<_, i32>>::from(longs).encode(writer)
            }
            Self::Float(floats) => {
                NbtTag::Float.encode(writer)?;
                <&Counted<_, i32>>::from(floats).encode(writer)
            }
            Self::Double(doubles) => {
                NbtTag::Double.encode(writer)?;
                <&Counted<_, i32>>::from(doubles).encode(writer)
            }
            Self::ByteArray(bytearrays) => {
                NbtTag::ByteArray.encode(writer)?;
                i32::try_from(bytearrays.len())?.encode(writer)?;
                for bytearray in bytearrays {
                    <&Counted<_, i32>>::from(bytearray).encode(writer)?;
                }
                Ok(())
            }
            Self::String(strings) => {
                NbtTag::String.encode(writer)?;
                i32::try_from(strings.len())?.encode(writer)?;
                for string in strings {
                    Mutf8::from(string).encode(writer)?;
                }
                Ok(())
            }
            Self::List(lists) => {
                NbtTag::List.encode(writer)?;
                <&Counted<_, i32>>::from(lists).encode(writer)
            }
            Self::Compound(compounds) => {
                NbtTag::Compound.encode(writer)?;
                <&Counted<_, i32>>::from(compounds).encode(writer)
            }
            Self::IntArray(intarrays) => {
                NbtTag::IntArray.encode(writer)?;
                i32::try_from(intarrays.len())?.encode(writer)?;
                for intarray in intarrays {
                    <&Counted<_, i32>>::from(intarray).encode(writer)?;
                }
                Ok(())
            }
            Self::LongArray(longarrays) => {
                NbtTag::LongArray.encode(writer)?;
                i32::try_from(longarrays.len())?.encode(writer)?;
                for longarray in longarrays {
                    <&Counted<_, i32>>::from(longarray).encode(writer)?;
                }
                Ok(())
            }
            Self::Invalid => {
                NbtTag::End.encode(writer)?;
                0u32.encode(writer)
            }
        }
    }
}

impl<'dec, 'a> Decode<'dec> for List<'a>
where
    'dec: 'a,
{
    fn decode(cursor: &mut std::io::Cursor<&'dec [u8]>) -> decode::Result<Self> {
        Ok(match NbtTag::decode(cursor)? {
            NbtTag::End => {
                if i32::decode(cursor)? > 0 {
                    return Err(decode::Error::Custom("TAG_End in List"));
                }
                List::Invalid
            }
            NbtTag::Byte => {
                let bytes = &<&Counted<[u8], i32>>::decode(cursor)?.inner;
                List::Byte(Cow::Borrowed(unsafe {
                    &*(bytes as *const [u8] as *const [i8])
                }))
            }
            NbtTag::Short => List::Short(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::Int => List::Int(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::Long => List::Long(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::Float => List::Float(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::Double => List::Double(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::ByteArray => List::ByteArray(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::String => List::String({
                let len = usize::try_from(i32::decode(cursor)?)?;
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    v.push(Mutf8::decode(cursor)?.0)
                }
                v
            }),
            NbtTag::List => List::List(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::Compound => List::Compound(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::IntArray => List::IntArray(<Counted<_, i32>>::decode(cursor)?.inner),
            NbtTag::LongArray => List::LongArray(<Counted<_, i32>>::decode(cursor)?.inner),
        })
    }
}
