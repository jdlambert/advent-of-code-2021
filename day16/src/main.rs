enum InnerPacket {
    Literal(u64),
    Operator { opcode: u64, packets: Vec<Packet> },
}

struct Packet {
    version: u64,
    inner: InnerPacket,
}

struct PacketParser {
    binary: String,
    i: usize,
}

impl Packet {
    fn from_str(binary: &str) -> Self {
        let mut parser = PacketParser::new(binary);
        parser.parse()
    }
}

impl PacketParser {
    fn new(binary: &str) -> Self {
        PacketParser {
            binary: binary.to_owned(),
            i: 0,
        }
    }

    fn read(&mut self, take: usize) -> u64 {
        let value = u64::from_str_radix(&self.binary[self.i..self.i + take], 2).unwrap();
        self.i += take;
        value
    }

    fn parse_literal(&mut self, version: u64) -> Packet {
        let mut literal = 0;
        while self.read(1) == 1 {
            literal = (literal << 4) | self.read(4);
        }
        literal = (literal << 4) | self.read(4);
        Packet {
            version,
            inner: InnerPacket::Literal(literal),
        }
    }

    fn parse_operator(&mut self, version: u64, opcode: u64) -> Packet {
        if self.read(1) == 1 {
            let packet_count = self.read(11);
            let packets = (0..packet_count).map(|_| self.parse()).collect();

            Packet {
                version,
                inner: InnerPacket::Operator { opcode, packets },
            }
        } else {
            let ending_bit = self.i + self.read(15) as usize;
            let mut packets = vec![];
            while self.i < ending_bit {
                packets.push(self.parse());
            }
            Packet {
                version,
                inner: InnerPacket::Operator { opcode, packets },
            }
        }
    }

    fn parse(&mut self) -> Packet {
        let version = self.read(3);
        let opcode = self.read(3);

        if opcode == 4 {
            self.parse_literal(version)
        } else {
            self.parse_operator(version, opcode)
        }
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.inner {
        InnerPacket::Literal(v) => *v,
        InnerPacket::Operator { opcode, packets } => {
            let mut packets = packets.iter().map(evaluate);
            match opcode {
                0 => packets.sum(),
                1 => packets.product(),
                2 => packets.min().unwrap(),
                3 => packets.max().unwrap(),
                5..=7 => {
                    let (a, b) = (packets.next(), packets.next());
                    if match opcode {
                        5 => a > b,
                        6 => a < b,
                        7 => a == b,
                        _ => unreachable!(),
                    } {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version
        + if let InnerPacket::Operator { opcode: _, packets } = &packet.inner {
            packets.iter().map(|p| version_sum(p)).sum()
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
    let packet = Packet::from_str(
        &include_str!("../input.txt")
            .trim()
            .chars()
            .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
            .collect::<String>(),
    );
    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
}
