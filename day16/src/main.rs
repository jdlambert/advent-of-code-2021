enum InnerPacket {
    Literal(u64),
    Operator { opcode: u64, packets: Vec<Packet> },
}
struct Packet {
    version: u64,
    inner: InnerPacket,
}

fn parse_bin(binary: &str, skip: usize, take: usize) -> u64 {
    u64::from_str_radix(&binary.chars().skip(skip).take(take).collect::<String>(), 2).unwrap()
}

fn from_string(binary: &str) -> (Packet, usize) {
    let version = parse_bin(binary, 0, 3);
    let opcode = parse_bin(binary, 3, 3);

    if opcode == 4 {
        let mut i = 6;
        let mut literal = 0;
        while parse_bin(binary, i, 1) == 1 {
            literal = (literal << 4) | parse_bin(binary, i + 1, 4);
            i += 5;
        }
        literal = (literal << 4) | parse_bin(binary, i + 1, 4);
        i += 5;
        (
            Packet {
                version,
                inner: InnerPacket::Literal(literal),
            },
            i,
        )
    } else {
        let mut i = 6;
        let length_type_id = parse_bin(binary, i, 1);
        i += 1;
        if length_type_id == 1 {
            let packet_count = parse_bin(binary, i, 11);
            i += 11;
            let mut packets = vec![];
            for _ in 0..packet_count {
                let (subpacket, length) = from_string(&binary[i..]);
                packets.push(subpacket);
                i += length;
            }

            (
                Packet {
                    version,
                    inner: InnerPacket::Operator { opcode, packets },
                },
                i,
            )
        } else {
            let bit_count = parse_bin(binary, i, 15) as usize;
            i += 15;
            let ending_bit = i + bit_count;
            let mut packets = vec![];
            while i < ending_bit {
                let (subpacket, length) = from_string(&binary[i..]);
                packets.push(subpacket);
                i += length;
            }
            (
                Packet {
                    version,
                    inner: InnerPacket::Operator { opcode, packets },
                },
                i,
            )
        }
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.inner {
        InnerPacket::Literal(v) => *v,
        InnerPacket::Operator { opcode, packets } => match opcode {
            0 => packets.iter().map(evaluate).sum(),
            1 => packets.iter().map(evaluate).product(),
            2 => packets.iter().map(evaluate).min().unwrap(),
            3 => packets.iter().map(evaluate).max().unwrap(),
            5 => {
                if evaluate(&packets[0]) > evaluate(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if evaluate(&packets[0]) < evaluate(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if evaluate(&packets[0]) == evaluate(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        },
    }
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version
        + if let InnerPacket::Operator { opcode: _, packets } = &packet.inner {
            let mut sum = 0; // TODO: why can't I get this to work as an iter???
            for p in packets {
                sum += version_sum(p);
            }
            sum
        } else {
            0
        }
}

fn part1(packet: &Packet) -> u64 {
    version_sum(packet)
}

fn part2(packet: &Packet) -> u64 {
    evaluate(packet)
}

fn main() {
    let binary = &include_str!("../input.txt")
        .trim()
        .chars()
        .map(|ch| ch.to_digit(16).unwrap())
        .map(|digit| format!("{:04b}", digit).chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>();
    let (packet, _) = from_string(binary);
    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
}
