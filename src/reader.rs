macro_rules! endian_mod {
    ($mod_name:ident, $to_bytes:ident, $from_bytes:ident) => {
        pub mod $mod_name {
            pub trait Reader {
                fn read_byte(&mut self) -> Result<u8, ()>;
                fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), ()> {
                    for b in bytes {
                        *b = self.read_byte()?;
                    }
                    Ok(())
                }
            }

            pub trait Readable: Sized {
                fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()>;
            }

            /// Reading from a u8 slice packs the bytes at its start and updates it to point to the
            /// remaining unread space.
            impl Reader for &[u8] {
                #[inline]
                fn read_byte(&mut self) -> Result<u8, ()> {
                    let (x, xs) = core::mem::replace(self, &[])
                        .split_first()
                        .ok_or(())?;
                    *self = xs;
                    Ok(x.clone())
                }

                #[inline]
                fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), ()> {
                    if bytes.len() > self.len() {
                        return Err(());
                    }

                    let (xs, ys) = core::mem::replace(self, &[]).split_at(bytes.len());
                    bytes.copy_from_slice(xs);
                    *self = ys;
                    Ok(())
                }
            }

            macro_rules! simple_num {
                ($numtype:ident, $n:literal) => {
                    impl Readable for $numtype {
                        fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()> {
                            let mut buf = [0u8; $n];
                            reader.read_bytes(&mut buf)?;
                            Ok($numtype::$from_bytes(buf))
                        }
                    }
                };
            }

            simple_num!(u8, 1);
            simple_num!(i8, 1);
            simple_num!(u16, 2);
            simple_num!(i16, 2);
            simple_num!(u32, 4);
            simple_num!(i32, 4);
            simple_num!(u64, 8);
            simple_num!(i64, 8);
            simple_num!(u128, 16);
            simple_num!(i128, 16);

            impl<T: Readable> Readable for [T; 1] {
                #[inline]
                fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()> {
                    Ok([T::read_from(reader)?])
                }
            }

            impl<T0: Readable, T1: Readable> Readable for (T0, T1) {
                #[inline]
                fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()> {
                    Ok((T0::read_from(reader)?, T1::read_from(reader)?))
                }
            }

            impl<T0: Readable, T1: Readable, T2: Readable> Readable for (T0, T1, T2) {
                #[inline]
                fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()> {
                    Ok((
                        T0::read_from(reader)?,
                        T1::read_from(reader)?,
                        T2::read_from(reader)?,
                    ))
                }
            }

            impl<T0: Readable, T1: Readable, T2: Readable, T3: Readable> Readable for (T0, T1, T2, T3) {
                #[inline]
                fn read_from<R: Reader>(reader: &mut R) -> Result<Self, ()> {
                    Ok((
                        T0::read_from(reader)?,
                        T1::read_from(reader)?,
                        T2::read_from(reader)?,
                        T3::read_from(reader)?,
                    ))
                }
            }
        }
    };
}

endian_mod!(le, to_le_bytes, from_le_bytes);
endian_mod!(be, to_be_bytes, from_be_bytes);
