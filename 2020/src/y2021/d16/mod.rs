use crate::support::*;
use data_encoding::HEXUPPER;
use bitreader::{BitReader, BitReaderError};

enum Packet
{
    Literal
    {
        version: u8,
        literal: u64,
    },
    Operator
    {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>,
    }
}

impl Packet
{
    fn version_sum(&self) -> u64
    {
        match self
        {
            Packet::Literal{ version, .. } =>
            {
                *version as u64
            },
            Packet::Operator{ version, sub_packets, .. } =>
            {
                (*version as u64) + sub_packets.iter().map(|p| p.version_sum()).sum::<u64>()
            },
        }
    }

    fn calculate(&self) -> u64
    {
        match self
        {
            Packet::Literal{ literal, .. } =>
            {
                *literal as u64
            },
            Packet::Operator{ type_id, sub_packets, .. } =>
            {
                match type_id
                {
                    0 => // Sum
                    {
                        sub_packets.iter()
                            .map(|p| p.calculate())
                            .sum::<u64>()
                    },
                    1 => // Product
                    {
                        sub_packets.iter()
                            .map(|p| p.calculate())
                            .product::<u64>()
                    },
                    2 => // Minimum
                    {
                        sub_packets.iter()
                            .map(|p| p.calculate())
                            .fold(u64::MAX, |a, b| u64::min(a, b))
                    },
                    3 => // Maximum
                    {
                        sub_packets.iter()
                            .map(|p| p.calculate())
                            .fold(u64::MIN, |a, b| u64::max(a, b))
                    },
                    5 => // Greater Than
                    {
                        assert!(sub_packets.len() == 2);
                        if sub_packets[0].calculate() > sub_packets[1].calculate() { 1 } else { 0 }
                    },
                    6 => // Less Than
                    {
                        assert!(sub_packets.len() == 2);
                        if sub_packets[0].calculate() < sub_packets[1].calculate() { 1 } else { 0 }
                    },
                    7 => // Equal To
                    {
                        assert!(sub_packets.len() == 2);
                        if sub_packets[0].calculate() == sub_packets[1].calculate() { 1 } else { 0 }
                    },
                    _ =>
                    {
                        unreachable!();
                    },
                }
            },
        }
    }
}

fn hex_to_bytes(input: &str) -> Vec<u8>
{
    let first_line = input_to_lines(input)[0].clone();
    HEXUPPER.decode(&first_line.chars().map(|c| c as u8).collect::<Vec<_>>()).unwrap()    
}

fn parse_packet<'a>(bits: &mut BitReader<'a>) -> Result<Packet, BitReaderError>
{
    let version = bits.read_u8(3)?;
    let type_id = bits.read_u8(3)?;

    match type_id
    {
        4 =>
        {
            // Literal: 5 bit values, each containing continuation bit + 4 bits data
            let mut literal: u64 = 0;
            for _ in 0..16
            {
                let part = bits.read_u8(5)?;
                
                literal = (literal << 4) | ((part & 0x0f) as u64);

                if (part & 0x10) == 0
                {
                    return Ok(Packet::Literal { version, literal });
                }
            }
            panic!("Too many bits in literal");
        },
        _ =>
        {
            // Operator - folled by 1 bit length type ID

            let length_type_id = bits.read_bool()?;

            if length_type_id
            {
                // Length type ID = 1
                // Operator, 11-bits = num sub-packets

                let num_sub_packets = bits.read_u16(11)?;

                let mut sub_packets = Vec::new();

                for _ in 0..num_sub_packets
                {
                    sub_packets.push(parse_packet(bits)?);
                }

                return Ok(Packet::Operator{ version, type_id, sub_packets });
            }
            else // length_type_id == 0
            {
                // Operator, 15-bits = total sub-packet bit length

                let len_bits = bits.read_u64(15)?;

                let target_remaining = bits.remaining() - len_bits;

                let mut sub_packets = Vec::new();

                while bits.remaining() > target_remaining
                {
                    sub_packets.push(parse_packet(bits)?);
                }

                return Ok(Packet::Operator{ version, type_id, sub_packets });
            }
        },
    }
}

fn parse(input: &str) -> Packet
{
    let bytes = hex_to_bytes(input);
    let mut bits = BitReader::new(&bytes);

    parse_packet(&mut bits).unwrap()
}

fn part_1(input: &str) -> u64
{
    parse(input).version_sum()
}

fn part_2(input: &str) -> u64
{
    parse(input).calculate()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(16)
        .example(|| Answer {
            calculated: part_1("D2FE28"),
            expected: 6,
        })
        .example(|| Answer {
            calculated: part_1("8A004A801A8002F478"),
            expected: 16,
        })
        .example(|| Answer {
            calculated: part_1("620080001611562C8802118E34"),
            expected: 12,
        })
        .example(|| Answer {
            calculated: part_1("C0015000016115A2E0802F182340"),
            expected: 23,
        })
        .example(|| Answer {
            calculated: part_1("A0016C880162017C3686B18A3D4780"),
            expected: 31,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 955,
        })
        .example(|| Answer {
            calculated: part_2("C200B40A82"),
            expected: 3,
        })
        .example(|| Answer {
            calculated: part_2("04005AC33890"),
            expected: 54,
        })
        .example(|| Answer {
            calculated: part_2("880086C3E88112"),
            expected: 7,
        })
        .example(|| Answer {
            calculated: part_2("CE00C43D881120"),
            expected: 9,
        })
        .example(|| Answer {
            calculated: part_2("D8005AC2A8F0"),
            expected: 1,
        })
        .example(|| Answer {
            calculated: part_2("F600BC2D8F"),
            expected: 0,
        })
        .example(|| Answer {
            calculated: part_2("9C005AC2F8F0"),
            expected: 0,
        })
        .example(|| Answer {
            calculated: part_2("9C0141080250320F1802104A08"),
            expected: 1,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 158135423448u64,
        })
}
