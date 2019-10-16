use std::cmp;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use vek::*;

/*
A region owns the Values from in (0, 2048) in steps of 1/32.
But because regions can also subscribe we add support to the range (0, 2048*3).
which is 13 bits for the digits before the decimal point and 5 bits for the digits after the decimal point.
We use our own LodPos type to store and compute based on these values, because u16 arithmetic (inside the owned area) is super easy to archive and allows us to optimize a lot.


-- lower neighbor
0 -> 0
65535 -> 2047 31/32
-- owned
65536 -> 2048
131071 -> 4095 31/32
-- upper neighbor
196607 -> 6143 31/32
*/

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct LodPos {
    /*
        bit 0..17 -> x
        bit 18..35 -> y
        bit 36..53 -> z
        bit 54..63 -> unused
    */
    data: u64,
}

/*does not work on big endian!*/
const BIT_X_MASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011_1111_1111_1111_1111;
const BIT_Y_MASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1100_0000_0000_0000_0000;
const BIT_Z_MASK: u64 =
    0b0000_0000_0011_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const BIT_X_MASK32: u32 = 0b0000_0000_0000_0011_1111_1111_1111_1111;
const BIT_Y_OFFSET: u8 = 18;
const BIT_Z_OFFSET: u8 = 36;

//TODO: Optimize!
impl LodPos {
    pub fn new(data: Vec3<u32>) -> Self {
        let mut index = LodPos { data: 0 };
        index.set(data);
        index
    }

    pub fn xyz(x: u32, y: u32, z: u32) -> Self {
        LodPos {
            data: Self::encode(&x, &y, &z),
        }
    }

    pub fn get(&self) -> Vec3<u32> {
        Vec3::from(Self::decode(&self.data))
    }

    pub fn set(&mut self, data: Vec3<u32>) {
        self.data = Self::encode(&data.x, &data.y, &data.z);
    }

    fn encode(x: &u32, y: &u32, z: &u32) -> u64 {
        let x = (x & BIT_X_MASK32) as u64;
        let y = ((y & BIT_X_MASK32) as u64) << BIT_Y_OFFSET;
        let z = ((z & BIT_X_MASK32) as u64) << BIT_Z_OFFSET;
        x + y + z
    }

    fn decode(data: &u64) -> (u32, u32, u32) {
        let x = (data & BIT_X_MASK) as u32;
        let y = ((data & BIT_Y_MASK) >> BIT_Y_OFFSET) as u32;
        let z = ((data & BIT_Z_MASK) >> BIT_Z_OFFSET) as u32;
        (x, y, z)
    }

    pub fn align_to_level(&self, layer: u8) -> LodPos {
        let xyz = self.get();
        let f = two_pow_u(layer) as u32;
        LodPos::new(xyz.map(|i| (i / f) * f))
    }

    pub fn get_highest_level_that_fits(&self) -> u8 {
        let pos = self.get();
        cmp::min(
            cmp::min(
                cmp::min(pos[0].trailing_zeros(), pos[1].trailing_zeros()),
                pos[2].trailing_zeros(),
            ),
            15,
        ) as u8
    }
}

impl fmt::Display for LodPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let xyz = self.get();
        //write!(f, "({}|{}|{}) <{}>", xyz[0], xyz[1], xyz[2], self.data)
        write!(f, "({}|{}|{})", xyz[0], xyz[1], xyz[2])
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct AbsIndex {
    pub layer: u8,
    pub index: usize,
}

impl AbsIndex {
    pub fn new(layer: u8, index: usize) -> Self {
        AbsIndex { layer, index }
    }
}

impl fmt::Display for AbsIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.layer, self.index)
    }
}

impl Sub for LodPos {
    type Output = LodPos;
    fn sub(self, rhs: LodPos) -> Self::Output {
        LodPos {
            data: self.data - rhs.data, /*fast but has overflow issues*/
        }
    }
}

impl Add for LodPos {
    type Output = LodPos;
    fn add(self, rhs: LodPos) -> Self::Output {
        LodPos {
            data: self.data + rhs.data, /*fast but has overflow issues*/
        }
    }
}

pub const fn two_pow_u(n: u8) -> u16 {
    1 << n
}
pub const fn two_pow_u32(n: u8) -> u32 {
    1 << n
}

pub const fn multily_with_2_pow_n(a: usize, n: u8) -> usize {
    //return a * 2^n but fast
    a << n
}

pub fn relative_to_1d(
    child_lod: LodPos,
    parent_lod: LodPos,
    child_layer: u8,
    relative_size: Vec3<u32>,
) -> usize {
    let width = two_pow_u32(child_layer) as u32;
    let index = (child_lod.get() - parent_lod.get()).map(|e| e / width);
    (index[0] * relative_size[2] * relative_size[1] + index[1] * relative_size[2] + index[2])
        as usize
}

pub fn min(lhs: LodPos, rhs: LodPos) -> LodPos {
    let lhs = lhs.get();
    let rhs = rhs.get();
    LodPos::new(lhs.map2(rhs, |a, b| cmp::min(a, b)))
}

pub fn max(lhs: LodPos, rhs: LodPos) -> LodPos {
    let lhs = lhs.get();
    let rhs = rhs.get();
    LodPos::new(lhs.map2(rhs, |a, b| cmp::max(a, b)))
}

/*************
    TESTS
**************/

#[cfg(test)]
mod tests {
    use crate::{lodstore::lodpos::two_pow_u32, lodstore::lodpos::LodPos};
    use test::Bencher;
    use vek::*;

    #[test]
    fn setter_getter() {
        let i = LodPos::xyz(0, 0, 0);
        assert_eq!(i.get(), Vec3::new(0, 0, 0));

        let i = LodPos::xyz(1337, 0, 0);
        assert_eq!(i.get(), Vec3::new(1337, 0, 0));

        let i = LodPos::xyz(0, 1337, 0);
        assert_eq!(i.get(), Vec3::new(0, 1337, 0));

        let i = LodPos::xyz(0, 0, 1337);
        assert_eq!(i.get(), Vec3::new(0, 0, 1337));

        let i = LodPos::xyz(1, 1, 1);
        assert_eq!(i.get(), Vec3::new(1, 1, 1));

        let i = LodPos::xyz(262143, 262143, 262143);
        assert_eq!(i.get(), Vec3::new(262143, 262143, 262143));

        let i = LodPos::xyz(262144, 262144, 262144); //overflow
        assert_eq!(i.get(), Vec3::new(0, 0, 0));

        let i = LodPos::xyz(42, 1337, 69);
        assert_eq!(i.get(), Vec3::new(42, 1337, 69));
    }

    #[test]
    fn align() {
        let i = LodPos::xyz(1337, 0, 0).align_to_level(4);
        assert_eq!(i.get(), Vec3::new(1328, 0, 0));

        let i = LodPos::xyz(1337, 1800, 0).align_to_level(5);
        assert_eq!(i.get(), Vec3::new(1312, 1792, 0));

        let i = LodPos::xyz(1337, 0, 50).align_to_level(3);
        assert_eq!(i.get(), Vec3::new(1336, 0, 48));

        let i = LodPos::xyz(1335, 0, 0).align_to_level(3);
        assert_eq!(i.get(), Vec3::new(1328, 0, 0));

        let i = LodPos::xyz(31337, 22000, 25000).align_to_level(7);
        assert_eq!(i.get(), Vec3::new(31232, 21888, 24960));

        let i = LodPos::xyz(31337, 22000, 25000).align_to_level(0);
        assert_eq!(i.get(), Vec3::new(31337, 22000, 25000));

        let i = LodPos::xyz(0, 0, 0).align_to_level(4);
        assert_eq!(i.get(), Vec3::new(0, 0, 0));
    }

    #[test]
    fn get_highest_level_that_fits() {
        let i = LodPos::xyz(0, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 15);
        let i = LodPos::xyz(1, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 0);
        let i = LodPos::xyz(2, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 1);
        let i = LodPos::xyz(3, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 0);
        let i = LodPos::xyz(4, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 2);
        let i = LodPos::xyz(5, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 0);

        let i = LodPos::xyz(1337, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 0);

        let i = LodPos::xyz(1337, 1800, 0);
        assert_eq!(i.get_highest_level_that_fits(), 0);

        let i = LodPos::xyz(1338, 0, 50);
        assert_eq!(i.get_highest_level_that_fits(), 1);

        let i = LodPos::xyz(1336, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 3);

        let i = LodPos::xyz(31348, 22000, 25000);
        assert_eq!(i.get_highest_level_that_fits(), 2);

        let i = LodPos::xyz(0, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 15);

        let i = LodPos::xyz(65536, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 15);

        let i = LodPos::xyz(32768, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 15);

        let i = LodPos::xyz(16384, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 14);

        let i = LodPos::xyz(8192, 0, 0);
        assert_eq!(i.get_highest_level_that_fits(), 13);

        let i = LodPos::xyz(65536, 0, 8192);
        assert_eq!(i.get_highest_level_that_fits(), 13);
    }

    #[bench]
    fn bench_access_two_pow(b: &mut Bencher) {
        b.iter(|| two_pow_u32(6));
    }

    #[bench]
    fn bench_access_align(b: &mut Bencher) {
        let access = LodPos::xyz(0, 0, 0);
        b.iter(|| access.align_to_level(6));
    }
}
