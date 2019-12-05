#![no_std]

mod reader;
mod writer;

pub mod le {
    pub use crate::reader::le::*;
    pub use crate::writer::le::*;
}

pub mod be {
    pub use crate::reader::be::*;
    pub use crate::writer::be::*;
}

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
