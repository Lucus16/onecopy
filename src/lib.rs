#![no_std]

pub mod le {
    use core::mem::transmute;

    pub trait Writer {
        fn write_u8(&mut self, item: u8) -> &mut Self;
        fn write_u16(&mut self, item: u16) -> &mut Self;
        fn write_u32(&mut self, item: u32) -> &mut Self;
        fn write_u64(&mut self, item: u64) -> &mut Self;
        fn write_u128(&mut self, item: u128) -> &mut Self;
        fn write<T: Writable>(&mut self, item: T) -> &mut Self;
    }

    pub trait Writable {
        fn write_to<T: Writer>(self, writer: &mut T);
    }

    impl Writer for &mut [u8] {
        #[inline]
        fn write_u8(&mut self, item: u8) -> &mut Self {
            let (h, t) = core::mem::replace(self, &mut []).split_at_mut(1);
            h[0] = item;
            *self = t;
            self
        }

        #[inline]
        fn write_u16(&mut self, item: u16) -> &mut Self {
            let (h, t) = core::mem::replace(self, &mut []).split_at_mut(2);
            h.copy_from_slice(&item.to_le_bytes());
            *self = t;
            self
        }

        #[inline]
        fn write_u32(&mut self, item: u32) -> &mut Self {
            let (h, t) = core::mem::replace(self, &mut []).split_at_mut(4);
            h.copy_from_slice(&item.to_le_bytes());
            *self = t;
            self
        }

        #[inline]
        fn write_u64(&mut self, item: u64) -> &mut Self {
            let (h, t) = core::mem::replace(self, &mut []).split_at_mut(8);
            h.copy_from_slice(&item.to_le_bytes());
            *self = t;
            self
        }

        #[inline]
        fn write_u128(&mut self, item: u128) -> &mut Self {
            let (h, t) = core::mem::replace(self, &mut []).split_at_mut(16);
            h.copy_from_slice(&item.to_le_bytes());
            *self = t;
            self
        }

        #[inline]
        fn write<T: Writable>(&mut self, item: T) -> &mut Self {
            item.write_to(self);
            self
        }
    }

    impl Writable for u8 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u8(self);
        }
    }

    impl Writable for i8 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u8(unsafe { transmute(self) });
        }
    }

    impl Writable for u16 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u16(self);
        }
    }

    impl Writable for i16 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u16(unsafe { transmute(self) });
        }
    }

    impl Writable for u32 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u32(self);
        }
    }

    impl Writable for i32 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u32(unsafe { transmute(self) });
        }
    }

    impl Writable for f32 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u32(unsafe { transmute(self) });
        }
    }

    impl Writable for u64 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u64(self);
        }
    }

    impl Writable for i64 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u64(unsafe { transmute(self) });
        }
    }

    impl Writable for f64 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u64(unsafe { transmute(self) });
        }
    }

    impl Writable for u128 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u128(self);
        }
    }

    impl Writable for i128 {
        #[inline]
        fn write_to<T: Writer>(self, writer: &mut T) {
            writer.write_u128(unsafe { transmute(self) });
        }
    }

    impl<T: Writable + Copy> Writable for &T {
        #[inline]
        fn write_to<W: Writer>(self, writer: &mut W) {
            (*self).write_to(writer);
        }
    }

    impl<T: Writable + Copy> Writable for &mut T {
        #[inline]
        fn write_to<W: Writer>(self, writer: &mut W) {
            (*self).write_to(writer);
        }
    }

    impl<T: Writable + Copy> Writable for &[T] {
        #[inline]
        fn write_to<W: Writer>(self, writer: &mut W) {
            for item in self {
                item.write_to(writer);
            }
        }
    }

    impl<T: Writable + Copy> Writable for &mut [T] {
        #[inline]
        fn write_to<W: Writer>(self, writer: &mut W) {
            for item in self {
                item.write_to(writer);
            }
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn write_uints() {
        use crate::le::Writer;
        let mut arr1 = [0u8; 15];
        let mut arr2 = [0u8; 15];
        let mut buf1: &mut [u8] = &mut arr1;
        let mut buf2: &mut [u8] = &mut arr2;
        buf1.write_u8(0x01);
        buf1.write_u16(0x2312);
        buf1.write_u32(0x67564534);
        buf1.write_u64(0xefdecdbcab9a8978);
        buf2.write_u64(0x7867564534231201);
        buf2.write_u32(0xbcab9a89);
        buf2.write_u16(0xdecd);
        buf2.write_u8(0xef);
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
        let mut foo: u16 = some_u16();
        let mut arr = [0u8; 16];
        let mut buf: &mut [u8] = &mut arr;
        buf.write(foo);
        buf.write(&foo);
        buf.write(&mut foo);
        buf.write(&(&foo, &foo));
        assert_eq!(
            arr,
            [0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0, 0, 0, 0, 0, 0]
        );
    }
}
