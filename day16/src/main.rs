use bits::BitReader;

use crate::Packet::LiteralPacket;

mod bits;

fn main() {
    let input = include_str!("../resources/input").trim();
    println!("[1/2] Result: {}", sum_version_numbers_for_input(input));
    println!("[2/2] Result: {}", evaluate_input(input));
}

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LesserThan,
    EqualTo,
}

impl From<u8> for Operation {
    fn from(ordinal: u8) -> Self {
        match ordinal {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LesserThan,
            7 => Operation::EqualTo,
            _ => panic!("Invalid ordinal '{}'", ordinal)
        }
    }
}

#[derive(Debug)]
enum Packet {
    LiteralPacket {
        version: u8,
        value: u64,
    },
    OperatorPacket {
        version: u8,
        operation: Operation,
        packets: Vec<Packet>,
    },
}

fn parse_literal_packet(version: u8, bit_reader: &mut BitReader) -> Option<(usize, Packet)> {
    let mut nibbles: Vec<u8> = vec![];

    loop {
        let group = bit_reader.read(5)?;
        nibbles.push(group & 0b00001111);

        if (group & 0b00010000) == 0 {
            break;
        }
    }

    let num_nibbles = nibbles.len();

    let mut value: u64 = 0;
    for i in 0..num_nibbles {
        value = value | (nibbles[i] as u64) << ((num_nibbles - 1) - i) * 4;
    }

    let bits_read = num_nibbles * 5;

    Some((
        bits_read,
        LiteralPacket {
            version,
            value,
        }
    ))
}

fn parse_operator_packet(packet_type: u8, version: u8, bit_reader: &mut BitReader) -> Option<(usize, Packet)> {
    let indicator = bit_reader.read(1)?;

    let mut bits_read: usize = 1;
    let mut packets: Vec<Packet> = vec![];

    if indicator == 0 {
        bits_read += 15;

        let length_in_bits = bit_reader.read_u64(15)? as usize;

        let mut nested_bits_read = 0;

        loop {
            let (nbits, packet) = parse_packet(bit_reader)?;
            packets.push(packet);
            nested_bits_read += nbits;

            if nested_bits_read >= length_in_bits {
                break;
            }
        }

        bits_read += nested_bits_read;
    } else {
        let length_in_packets = bit_reader.read_u64(11)?;
        bits_read += 11;

        let mut nested_bits_read = 0;

        for _ in 0..length_in_packets {
            let (nbits, packet) = parse_packet(bit_reader)?;
            packets.push(packet);
            nested_bits_read += nbits;
        }

        bits_read += nested_bits_read;
    }

    Some((
        bits_read,
        Packet::OperatorPacket {
            version,
            operation: packet_type.into(),
            packets,
        }
    ))
}


fn parse_packet(bit_reader: &mut BitReader) -> Option<(usize, Packet)> {
    let version = bit_reader.read(3).unwrap();
    let packet_type = bit_reader.read(3).unwrap();

    if let Some((bits_read, packet)) = match packet_type {
        4 => parse_literal_packet(version, bit_reader),
        _ => parse_operator_packet(packet_type, version, bit_reader),
    } {
        Some((6 + bits_read, packet))
    } else {
        None
    }
}

fn sum_version_numbers(packet: &Packet) -> usize {
    match packet {
        Packet::LiteralPacket { version, value } => *version as usize,
        Packet::OperatorPacket { version, operation, packets } => (*version as usize) + packets.iter().map(sum_version_numbers).sum::<usize>(),
    }
}

fn sum_version_numbers_for_input(input: &str) -> usize {
    let mut bit_reader = BitReader::new(input);
    let packet = parse_packet(&mut bit_reader).unwrap().1;
    sum_version_numbers(&packet)
}

fn value_for_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::LiteralPacket { version, value } => *value,

        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::Sum => packets.iter().map(value_for_packet).sum(),

        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::Product => packets.iter().map(value_for_packet).product(),

        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::Minimum => packets.iter().map(value_for_packet).min().unwrap(),

        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::Maximum => packets.iter().map(value_for_packet).max().unwrap(),

        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::GreaterThan => {
            let (a, b) = (&packets[0], &packets[1]);
            if value_for_packet(a) > value_for_packet(b) {
                1
            } else {
                0
            }
        }
        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::LesserThan => {
            let (a, b) = (&packets[0], &packets[1]);
            if value_for_packet(a) < value_for_packet(b) {
                1
            } else {
                0
            }
        }
        Packet::OperatorPacket { version, operation, packets }
        if *operation == Operation::EqualTo => {
            let (a, b) = (&packets[0], &packets[1]);
            if value_for_packet(a) == value_for_packet(b) {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid packet {:?}", packet)
    }
}

fn evaluate_input(input: &str) -> u64 {
    let mut bit_reader = BitReader::new(input);
    let packet = parse_packet(&mut bit_reader).unwrap().1;
    value_for_packet(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        assert_eq!(16, sum_version_numbers_for_input("8A004A801A8002F478"));
        assert_eq!(12, sum_version_numbers_for_input("620080001611562C8802118E34"));
        assert_eq!(23, sum_version_numbers_for_input("C0015000016115A2E0802F182340"));
        assert_eq!(31, sum_version_numbers_for_input("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(3, evaluate_input("C200B40A82"));
        assert_eq!(54, evaluate_input("04005AC33890"));
        assert_eq!(7, evaluate_input("880086C3E88112"));
        assert_eq!(9, evaluate_input("CE00C43D881120"));
        assert_eq!(1, evaluate_input("D8005AC2A8F0"));
        assert_eq!(0, evaluate_input("F600BC2D8F"));
        assert_eq!(0, evaluate_input("9C005AC2F8F0"));
        assert_eq!(1, evaluate_input("9C0141080250320F1802104A08"));
    }
}
