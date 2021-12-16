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

impl PacketParser {
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
        let mut packets = vec![];
        if self.read(1) == 1 {
            for _ in 0..self.read(11) {
                packets.push(self.parse())
            }
        } else {
            let ending_bit = self.read(15) as usize + self.i;
            while self.i < ending_bit {
                packets.push(self.parse());
            }
        }
        Packet {
            version,
            inner: InnerPacket::Operator { opcode, packets },
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

impl Packet {
    fn from_str(hex: &str) -> Self {
        let binary = hex
            .trim()
            .chars()
            .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
            .collect();
        let mut parser = PacketParser { binary, i: 0 };
        parser.parse()
    }

    fn evaluate(&self) -> u64 {
        match &self.inner {
            InnerPacket::Literal(v) => *v,
            InnerPacket::Operator { opcode, packets } => {
                let mut packets = packets.iter().map(|p| p.evaluate());
                match opcode {
                    0 => packets.sum(),
                    1 => packets.product(),
                    2 => packets.min().unwrap(),
                    3 => packets.max().unwrap(),
                    5..=7 => {
                        let (a, b) = (packets.next(), packets.next());
                        (match opcode {
                            5 => a > b,
                            6 => a < b,
                            7 => a == b,
                            _ => unreachable!(),
                        }) as u64
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        self.version
            + if let InnerPacket::Operator { packets, .. } = &self.inner {
                packets.iter().map(|p| p.version_sum()).sum()
            } else {
                0
            }
    }
}

fn part1(packet: &Packet) -> u64 {
    packet.version_sum()
}

fn part2(packet: &Packet) -> u64 {
    packet.evaluate()
}

fn main() {
    let packet = Packet::from_str(include_str!("../input.txt"));
    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
}
