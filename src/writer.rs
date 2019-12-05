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

            macro_rules! simple_num {
                ($numtype:ident) => {
                    impl Writable for $numtype {
                        #[inline]
                        fn write_to<W: Writer>(self, writer: &mut W) {
                            writer.write_slice(&self.$to_bytes())
                        }
                    }
                };
            }
            simple_num!(u8);
            simple_num!(i8);
            simple_num!(u16);
            simple_num!(i16);
            simple_num!(u32);
            simple_num!(i32);
            simple_num!(u64);
            simple_num!(i64);
            simple_num!(u128);
            simple_num!(i128);

            impl Writable for f32 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write::<u32>(unsafe { transmute(self) });
                }
            }

            impl Writable for f64 {
                #[inline]
                fn write_to<W: Writer>(self, writer: &mut W) {
                    writer.write::<u64>(unsafe { transmute(self) });
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
