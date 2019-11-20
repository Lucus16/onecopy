#![no_std]

macro_rules! endian_mod {
    ($mod_name:ident, $to_bytes:ident, $from_bytes:ident) => {
        pub mod $mod_name {
            use core::mem::transmute;

            pub trait Writer {
                fn write_slice(&mut self, items: &[u8]);
                fn write<T: Writable>(&mut self, item: T) -> &mut Self;
            }

            pub trait Writable: Copy {
                fn write_to<W: Writer>(self, writer: &mut W);

                #[inline]
                fn size(&self) -> usize {
                    *0.write(self)
                }
            }

            /// Writing to a u8 slice packs the bytes at its start and updates it to point to the
            /// remaining free space.
            impl Writer for &mut [u8] {
                #[inline]
                fn write_slice(&mut self, items: &[u8]) {
                    let (h, t) = core::mem::replace(self, &mut []).split_at_mut(items.len());
                    h.copy_from_slice(items);
                    *self = t;
                }

                #[inline]
                fn write<T: Writable>(&mut self, item: T) -> &mut Self {
                    item.write_to(self);
                    self
                }
            }

            /// Writing to a usize increases it by the number of bytes that would be written.
            impl Writer for usize {
                #[inline]
                fn write_slice(&mut self, items: &[u8]) {
                    *self += items.len();
                }

                #[inline]
                fn write<T: Writable>(&mut self, item: T) -> &mut Self {
                    item.write_to(self);
                    self
                }
            }

            impl Writable for u8 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for i8 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for u16 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for i16 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for u32 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for i32 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for f32 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write::<u32>(unsafe { transmute(self) });
                }
            }

            impl Writable for u64 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for i64 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for f64 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write::<u64>(unsafe { transmute(self) });
                }
            }

            impl Writable for u128 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl Writable for i128 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write_slice(&self.$to_bytes())
                }
            }

            impl<T: Writable> Writable for &T {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    (*self).write_to(writer);
                }
            }

            impl<T: Writable> Writable for &[T] {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    for item in self {
                        item.write_to(writer);
                    }
                }
            }

            impl<T: Writable> Writable for [T; 0] {
                #[inline]
                fn write_to<W: Writer>(self, _writer: &mut W) {}
            }

            impl<T: Writable> Writable for [T; 1] {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self[0].write_to(writer);
                }
            }

            impl<T: Writable> Writable for [T; 2] {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self[0].write_to(writer);
                    self[1].write_to(writer);
                }
            }

            impl<T: Writable> Writable for [T; 3] {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self[0].write_to(writer);
                    self[1].write_to(writer);
                    self[2].write_to(writer);
                }
            }

            impl<T: Writable> Writable for [T; 4] {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self[0].write_to(writer);
                    self[1].write_to(writer);
                    self[2].write_to(writer);
                    self[3].write_to(writer);
                }
            }

            impl Writable for () {
                #[inline]
                fn write_to<W: Writer>(self, _writer: &mut W) {}
            }

            impl<T0: Writable, T1: Writable> Writable for (T0, T1) {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self.0.write_to(writer);
                    self.1.write_to(writer);
                }
            }

            impl<T0, T1, T2> Writable for (T0, T1, T2)
            where
                T0: Writable,
                T1: Writable,
                T2: Writable,
            {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self.0.write_to(writer);
                    self.1.write_to(writer);
                    self.2.write_to(writer);
                }
            }

            impl<T0, T1, T2, T3> Writable for (T0, T1, T2, T3)
            where
                T0: Writable,
                T1: Writable,
                T2: Writable,
                T3: Writable,
            {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    self.0.write_to(writer);
                    self.1.write_to(writer);
                    self.2.write_to(writer);
                    self.3.write_to(writer);
                }
            }
        }
    };
}

endian_mod!(le, to_le_bytes, from_le_bytes);
endian_mod!(be, to_be_bytes, from_be_bytes);

#[cfg(not(target_endian = "little"))]
pub use be as ne;
#[cfg(target_endian = "little")]
pub use le as ne;

#[cfg(test)]
mod tests {
    #[test]
    fn write_uints() {
        use crate::le::Writer;
        let mut arr1 = [0u8; 15];
        let mut arr2 = [0u8; 15];
        let mut buf1: &mut [u8] = &mut arr1;
        let mut buf2: &mut [u8] = &mut arr2;
        buf1.write(0x01u8);
        buf1.write(0x2312u16);
        buf1.write(0x67564534u32);
        buf1.write(0xefdecdbcab9a8978u64);
        buf2.write(0x7867564534231201u64);
        buf2.write(0xbcab9a89u32);
        buf2.write(0xdecdu16);
        buf2.write(0xefu8);
        assert_eq!(buf1, []);
        assert_eq!(buf2, []);
        assert_eq!(arr1, arr2);
    }

    #[inline(never)]
    fn some_u16() -> u16 {
        let x = 0x1337;
        unsafe { core::ptr::read_volatile(&x) }
    }

    #[test]
    fn mix_ref_types() {
        use crate::le::Writer;
        let foo: u16 = some_u16();
        let mut arr = [0u8; 16];
        let mut buf: &mut [u8] = &mut arr;
        buf.write(foo);
        buf.write((&foo, ()));
        buf.write(&(&[foo, foo], [foo]));
        assert_eq!(
            arr,
            [0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0, 0, 0, 0, 0, 0]
        );
    }
}
