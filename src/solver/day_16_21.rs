use std::io::BufRead;
use enumset::__internal::EnumSetTypeRepr;
use num::PrimInt;

use crate::day::CombinedSolver;

pub struct Day1621;

impl CombinedSolver for Day1621 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let str = input.lines().next().unwrap().unwrap();
        let mut bit_stream: BitParser<_> = str
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .flat_map(|i| [i.has_bit(3), i.has_bit(2), i.has_bit(1), i.has_bit(0)])
            .into();
        
        let root = bit_stream.read_next();
        
        let part_1 = root.version_sum();
        let part_2 = root.value();

        Ok((part_1.to_string(), part_2.to_string()))
    }
}


#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketType,
}

impl Packet {
    fn version_sum(&self) -> usize {
        let mut sum = self.version as usize;
        
        match &self.data {
            PacketType::Literal(_) => {}
            PacketType::Operator(packets) => {
                sum += packets.iter().map(Packet::version_sum).sum::<usize>();
            }
        }
        
        sum
    }
    
    fn value(&self) -> usize {
        match &self.data {
            PacketType::Literal(n) => *n,
            PacketType::Operator(packets) => {
                match self.type_id {
                    0 => packets.iter().map(Packet::value).sum::<usize>(),
                    1 => packets.iter().map(Packet::value).product::<usize>(),
                    2 => packets.iter().map(Packet::value).min().unwrap(),
                    3 => packets.iter().map(Packet::value).max().unwrap(),
                    5 => (packets.first().unwrap().value() > packets.last().unwrap().value()) as usize,
                    6 => (packets.first().unwrap().value() < packets.last().unwrap().value()) as usize,
                    7 => (packets.first().unwrap().value() == packets.last().unwrap().value()) as usize,
                    c => panic!("{c}")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

struct BitParser<IT: Iterator<Item=bool>> {
    it: IT,
    i: usize,
}

impl<IT: Iterator<Item=bool>> From<IT> for BitParser<IT> {
    fn from(value: IT) -> Self {
        Self {
            it: value,
            i: 0,
        }
    }
}

impl<IT: Iterator<Item=bool>> BitParser<IT> {
    fn next_n<T: PrimInt>(&mut self, n: u16) -> T {
        let mut out = T::zero();

        for _ in 0..n {
            out = out.shl(1);
            if self.next_bit() {
                out = out + T::one();
            }
        }

        out
    }

    fn next_bit(&mut self) -> bool {
        self.i += 1;
        self.it.next().expect("Ran out of bits")
    }

    fn read_next(&mut self) -> Packet {
        let version = self.next_n::<u8>(3);
        let type_id = self.next_n::<u8>(3);

        let data = if type_id == 4 {  // literal
            let mut value = 0;
            loop {
                let b = self.next_bit();
                value = (value << 4) + self.next_n::<usize>(4);
                if !b { break; }
            }
            PacketType::Literal(value)
        } else {  // operator
            let lt_id = self.next_bit();
            let packets = if lt_id {  // num packets
                let num_sub_packets: u16 = self.next_n(11);
                (0..num_sub_packets)
                    .map(|_| self.read_next())
                    .collect()
            } else {  // num bits
                let num_bits = self.next_n(15);
                let current = self.i;
                let mut packets = Vec::new();
                while (self.i - current) < num_bits {
                    packets.push(self.read_next());
                }
                packets
            };
            
            PacketType::Operator(packets)
        };

        Packet {
            version,
            type_id,
            data,
        }
    }
}
