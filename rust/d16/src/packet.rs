pub trait BitReader {
    fn read(&mut self, n: u32) -> Option<u32>;
}

pub trait ReadFrom<T: BitReader> {
    fn read_from(reader: &mut T) -> Self where Self: Sized;
}

pub struct PacketHeader {
    pub version: Version,
    pub packet_type: PacketType,
}

impl<T: BitReader> ReadFrom<T> for PacketHeader {
    fn read_from(reader: &mut T) -> Self {
        let version = Version::read_from(reader);
        let packet_type = PacketType::read_from(reader);
        PacketHeader { version, packet_type }
    }
}

pub struct Version(u32);

impl Into<u32> for Version {
    fn into(self) -> u32 {
        self.0
    }
}

impl<T: BitReader> ReadFrom<T> for Version {
    fn read_from(reader: &mut T) -> Self {
        Version(reader.read(3).unwrap())
    }
}

pub struct Length {
    pub length_type: LengthType,
    pub length: u32,
}

impl<T: BitReader> ReadFrom<T> for Length {
    fn read_from(reader: &mut T) -> Self {
        let length_type = LengthType::read_from(reader);
        let length = match length_type {
            LengthType::Bits => reader.read(15).unwrap(),
            LengthType::Packets => reader.read(11).unwrap(),
        };
        Length { length_type, length }
    }
}

#[derive(Copy, Clone)]
pub enum LengthType {
    Bits,
    Packets,
}

impl<T: BitReader> ReadFrom<T> for LengthType {
    fn read_from(reader: &mut T) -> Self {
        match reader.read(1).unwrap() {
            0 => LengthType::Bits,
            1 => LengthType::Packets,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl<T: BitReader> ReadFrom<T> for PacketType {
    fn read_from(reader: &mut T) -> Self {
        let id = reader.read(3).unwrap();
        match id {
            4 => PacketType::Literal,
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::EqualTo,
            _ => unreachable!(),
        }
    }
}

pub enum LiteralGroup {
    More(u32),
    Final(u32),
}

impl<T: BitReader> ReadFrom<T> for LiteralGroup {
    fn read_from(reader: &mut T) -> Self {
        let literal_type = reader.read(1).unwrap();
        let value = reader.read(4).unwrap();
        match literal_type {
            1 => LiteralGroup::More(value),
            0 => LiteralGroup::Final(value),
            _ => unreachable!(),
        }
    }
}

pub struct Literal(u64);

impl<T: BitReader> ReadFrom<T> for Literal {
    fn read_from(reader: &mut T) -> Self {
        let mut output = 0u64;
        loop {
            let group = LiteralGroup::read_from(reader);
            match group {
                LiteralGroup::More(v) => {
                    output |= v as u64;
                    output <<= 4;
                }
                LiteralGroup::Final(v) => {
                    output |= v as u64;
                    return Literal(output);
                }
            }
        }
    }
}

impl Into<u64> for Literal {
    fn into(self) -> u64 {
        self.0
    }
}
