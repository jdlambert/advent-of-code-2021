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
            let bit_count = self.read(15) as usize;
            let ending_bit = self.i + bit_count;
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
    let mut parser = PacketParser::new(binary);
    let packet = parser.parse();
    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
}
