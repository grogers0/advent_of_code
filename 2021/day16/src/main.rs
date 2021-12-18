use std::io::{self, Read};

struct BitString(Vec<u8>);

impl BitString {
    fn from_hex(s: &str) -> Self {
        fn hex_ch(ch: u8) -> u8 {
            match ch {
                b'A'..=b'F' => ch - b'A' + 10,
                b'0'..=b'9' => ch - b'0',
                _ => panic!()
            }
        }
        let s = s.trim_end();
        if s.len() % 2 != 0 { panic!() }

        let data = s.as_bytes()
            .chunks(2)
            .map(|pair| hex_ch(pair[0]) << 4 | hex_ch(pair[1]))
            .collect();
        BitString(data)
    }

    fn iter<'a>(&'a self) -> BitStringIter<'a> {
        BitStringIter { data: &self.0, idx: 0 }
    }
}

struct BitStringIter<'a> {
    data: &'a [u8],
    idx: usize
}

impl <'a> Iterator for BitStringIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.idx / 8 >= self.data.len() {
            return None
        }
        let byte_idx = self.idx / 8;
        let bit_idx = 7 - (self.idx % 8);
        let ret = (self.data[byte_idx] & (1 << bit_idx)) != 0;
        self.idx += 1;
        Some(ret)
    }
}

impl <'a> BitStringIter<'a> {
    fn read_u8(&mut self, nbits: usize) -> u8 {
        assert!(nbits <= 8 && nbits > 0);
        let mut ret = 0;
        for _ in 0..nbits {
            ret = (ret << 1) | if self.next().unwrap() { 1 } else { 0 };
        }
        ret
    }

    fn read_u16(&mut self, nbits: usize) -> u16 {
        assert!(nbits <= 16 && nbits > 0);
        let mut ret = 0;
        for _ in 0..nbits {
            ret = (ret << 1) | if self.next().unwrap() { 1 } else { 0 };
        }
        ret
    }

    fn assert_padding(&mut self) {
        while let Some(bit) = self.next() {
            assert_eq!(false, bit);
        }
    }
}

struct Packet {
    version: u8,
    contents: PacketContents,
}

impl Packet {
    fn parse(puzzle_input: &str) -> Self {
        let bits = BitString::from_hex(puzzle_input);
        let mut bits = bits.iter();
        let ret = Packet::parse_from_bits(&mut bits);
        bits.assert_padding();
        ret
    }

    fn parse_from_bits(bits: &mut BitStringIter) -> Self {
        let version = bits.read_u8(3);
        let type_id = bits.read_u8(3);
        if type_id == 4 {
            let mut value = 0u64;
            loop {
                let group = bits.read_u8(5);
                value = (value << 4) | (group & 0xf) as u64;
                if (group & 0x10) == 0 {
                    break
                }
            }
            let contents = PacketContents::Literal(value);
            Packet { version, contents }
        } else {
            let operator_type = OperatorType::from_type_id(type_id);
            let length_type = LengthType::parse_from_bits(bits);
            let length = match length_type {
                LengthType::TotalBits => bits.read_u16(15),
                LengthType::NumPackets => bits.read_u16(11),
            } as usize;
            let mut sub_packets = Vec::new();
            match length_type {
                LengthType::NumPackets => {
                    for _ in 0..length {
                        sub_packets.push(Packet::parse_from_bits(bits));
                    }
                },
                LengthType::TotalBits => {
                    let stop_idx = bits.idx + length;
                    while bits.idx < stop_idx {
                        sub_packets.push(Packet::parse_from_bits(bits));
                    }
                    assert_eq!(stop_idx, bits.idx);
                }
            }
            let contents = PacketContents::Operator(operator_type, sub_packets);
            Packet { version, contents }
        }
    }
}

enum PacketContents {
    Literal(u64),
    Operator(OperatorType, Vec<Packet>)
}

enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo
}

impl OperatorType {
    fn from_type_id(type_id: u8) -> OperatorType {
        match type_id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!()
        }
    }
}

enum LengthType {
    TotalBits, NumPackets
}

impl LengthType {
    fn parse_from_bits(bits: &mut BitStringIter) -> LengthType {
        if bits.next().unwrap() {
            LengthType::NumPackets
        } else {
            LengthType::TotalBits
        }
    }
}

fn part1(puzzle_input: &str) -> u64 {
    fn sum_versions(packet: &Packet) -> u64 {
        let mut sum = packet.version as u64;
        if let PacketContents::Operator(_, ref sub_packets) = packet.contents {
            for sub_packet in sub_packets {
                sum += sum_versions(sub_packet);
            }
        }
        sum
    }

    let packet = Packet::parse(puzzle_input);
    sum_versions(&packet)
}

fn part2(puzzle_input: &str) -> u64 {
    fn calc_with_operators(packet: &Packet) -> u64 {
        match &packet.contents {
            PacketContents::Literal(val) => *val,
            PacketContents::Operator(OperatorType::Sum, sub_packets) => {
                sub_packets.iter().map(|p| calc_with_operators(p)).sum()
            },
            PacketContents::Operator(OperatorType::Product, sub_packets) => {
                sub_packets.iter().map(|p| calc_with_operators(p)).product()
            },
            PacketContents::Operator(OperatorType::Minimum, sub_packets) => {
                sub_packets.iter().map(|p| calc_with_operators(p)).min().unwrap()
            },
            PacketContents::Operator(OperatorType::Maximum, sub_packets) => {
                sub_packets.iter().map(|p| calc_with_operators(p)).max().unwrap()
            },
            PacketContents::Operator(cmp_op, sub_packets) => {
                assert_eq!(2, sub_packets.len());
                let val_0 = calc_with_operators(&sub_packets[0]);
                let val_1 = calc_with_operators(&sub_packets[1]);
                let cmp_result = match cmp_op {
                    OperatorType::GreaterThan => val_0 > val_1,
                    OperatorType::LessThan => val_0 < val_1,
                    OperatorType::EqualTo => val_0 == val_1,
                    _ => panic!(),
                };
                if cmp_result { 1 } else { 0 }
            },
        }
    }

    let packet = Packet::parse(puzzle_input);
    calc_with_operators(&packet)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(matches!(
                Packet::parse("D2FE28"),
                Packet {
                    version: 6,
                    contents: PacketContents::Literal(2021),
                }));
        assert!(matches!(
                Packet::parse("38006F45291200"),
                Packet {
                    version: 1,
                    contents: PacketContents::Operator(OperatorType::LessThan, sub_packets),
                } if matches!(sub_packets.as_slice(), &[
                        Packet {
                            version: 6,
                            contents: PacketContents::Literal(10),
                        },
                        Packet {
                            version: 2,
                            contents: PacketContents::Literal(20),
                        },
                    ])));
        assert!(matches!(
                Packet::parse("EE00D40C823060"),
                Packet {
                    version: 7,
                    contents: PacketContents::Operator(OperatorType::Maximum, sub_packets),
                } if matches!(sub_packets.as_slice(), &[
                        Packet {
                            version: 2,
                            contents: PacketContents::Literal(1),
                        },
                        Packet {
                            version: 4,
                            contents: PacketContents::Literal(2),
                        },
                        Packet {
                            version: 1,
                            contents: PacketContents::Literal(3),
                        },
                    ])));
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));
        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(0, part2("9C005AC2F8F0"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}
