//! Walking the pack and packet chain of an MPEG program stream.
//!
//! A program stream is a flat list of packets, each opening with the three-byte start code
//! prefix `00 00 01` and a stream identifier. There is no index, no central directory and no
//! back-pointer anywhere: the only way to know where a packet ends is to read its length, and
//! the only way to find the next one is to have read the previous one correctly. That is what
//! makes the format both easy to walk and impossible to verify - nothing in the file restates
//! anything else, so a packet that parses cannot be checked against a second opinion.
//!
//! The walk is therefore used for exactly one thing: finding where the last packet that is
//! *entirely present* ends. That answers the question a truncated recording poses and nothing
//! more. Whether the pictures inside those packets decode is left to the decoder, which is the
//! only component that can actually tell.

use std::ops::Range;

use crate::file_fixer::io::read_u16_be;

const START_CODE: [u8; 3] = [0x00, 0x00, 0x01];

/// Stream identifiers that do not carry a 16-bit length after the start code.
const PACK_HEADER: u8 = 0xBA;
const PROGRAM_END: u8 = 0xB9;

/// The first identifier that is followed by a 16-bit length: the system header, then the
/// stream map and every elementary stream above it. Below it sit the pack header and the
/// program end code, handled separately, and beneath those the elementary-stream start codes -
/// sequence headers, pictures, slices - which a raw `.m2v` dump is made of and which carry no
/// length at all.
const FIRST_LENGTH_BEARING_ID: u8 = 0xBB;

/// Length of an MPEG-2 pack header before its stuffing bytes, and of an MPEG-1 one, which has
/// no stuffing. The two are told apart by the top bits of the byte after the start code.
const MPEG2_PACK_LEN: usize = 14;
const MPEG1_PACK_LEN: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub start: usize,
    pub len: usize,
    pub stream_id: u8,
    /// False when the packet's declared length runs past the end of the file.
    pub complete: bool,
}

impl Packet {
    pub fn end(&self) -> usize {
        self.start + self.len
    }

    pub fn is_pack_header(&self) -> bool {
        self.stream_id == PACK_HEADER
    }
}

/// Reads the packet starting at `offset`, or `None` when nothing valid starts there.
pub fn read_packet(data: &[u8], offset: usize) -> Option<Packet> {
    if data.get(offset..offset + 3)? != START_CODE {
        return None;
    }
    let stream_id = *data.get(offset + 3)?;
    let remaining = data.len() - offset;

    let len = match stream_id {
        PROGRAM_END => 4,
        PACK_HEADER => {
            let marker = *data.get(offset + 4)?;
            if marker & 0xC0 == 0x40 {
                // MPEG-2 carries a stuffing count in the low three bits of the last byte.
                let stuffing = usize::from(data.get(offset + MPEG2_PACK_LEN - 1)? & 0x07);
                MPEG2_PACK_LEN + stuffing
            } else if marker & 0xF0 == 0x20 {
                MPEG1_PACK_LEN
            } else {
                return None;
            }
        }
        id if id >= FIRST_LENGTH_BEARING_ID => 6 + usize::from(read_u16_be(data, offset + 4)?),
        // An elementary-stream start code. It has no length, so the chain cannot be walked.
        _ => return None,
    };
    if len < 4 {
        return None;
    }

    Some(Packet {
        start: offset,
        len,
        stream_id,
        complete: len <= remaining,
    })
}

pub struct Structure {
    pub packets: Vec<Packet>,
    /// End of the last packet that is entirely present.
    pub last_complete_end: usize,
    /// Bytes at the front that no packet accounted for.
    pub leading_junk: usize,
}

impl Structure {
    /// Walks the chain from the first start code, or `None` when the file does not open as a
    /// program stream at all.
    pub fn of(data: &[u8]) -> Option<Self> {
        let start = first_pack(data)?;
        let mut packets = Vec::new();
        let mut offset = start;
        let mut last_complete_end = start;

        while let Some(packet) = read_packet(data, offset) {
            offset = packet.end();
            let complete = packet.complete;
            packets.push(packet);
            if !complete {
                break;
            }
            last_complete_end = offset;
        }

        // One pack header proves very little - four bytes match by chance. A chain of packets
        // that each land exactly on the next one does not.
        if packets.len() < MIN_PACKETS {
            return None;
        }
        Some(Self {
            packets,
            last_complete_end,
            leading_junk: start,
        })
    }

    /// True when the chain accounts for the file from its first byte to its last.
    pub fn is_whole(&self, len: usize) -> bool {
        self.leading_junk == 0 && self.packets.iter().all(|packet| packet.complete) && self.last_complete_end == len
    }

    pub fn kept(&self) -> Range<usize> {
        self.leading_junk..self.last_complete_end
    }
}

/// How many chained packets a file has to hold before it is treated as a program stream.
const MIN_PACKETS: usize = 4;

/// Offsets past this point are not searched for the first pack header.
const PACK_SCAN_LIMIT: usize = 1 << 20;

/// Finds the first pack header, allowing for a carve that began mid-sector.
fn first_pack(data: &[u8]) -> Option<usize> {
    let window = data.get(..PACK_SCAN_LIMIT.min(data.len()))?;
    (0..window.len().saturating_sub(4)).find(|&offset| read_packet(data, offset).is_some_and(|packet| packet.is_pack_header()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mpeg2_pack(stuffing: usize) -> Vec<u8> {
        let mut pack = vec![0x00, 0x00, 0x01, PACK_HEADER, 0x44];
        pack.resize(MPEG2_PACK_LEN, 0);
        pack[MPEG2_PACK_LEN - 1] = 0xF8 | stuffing as u8;
        pack.resize(MPEG2_PACK_LEN + stuffing, 0xFF);
        pack
    }

    fn pes(stream_id: u8, payload_len: usize) -> Vec<u8> {
        let mut packet = vec![0x00, 0x00, 0x01, stream_id];
        packet.extend_from_slice(&(payload_len as u16).to_be_bytes());
        packet.resize(6 + payload_len, 0);
        packet
    }

    fn stream(packs: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for _ in 0..packs {
            data.extend_from_slice(&mpeg2_pack(3));
            // The system header sits between the pack header and the first elementary packet,
            // and it carries a length like a PES packet does - a walk that treats it as an
            // elementary start code stops on the second packet of every real file.
            data.extend_from_slice(&pes(0xBB, 12));
            data.extend_from_slice(&pes(0xE0, 200));
            data.extend_from_slice(&pes(0xC0, 64));
        }
        data.extend_from_slice(&[0x00, 0x00, 0x01, PROGRAM_END]);
        data
    }

    #[test]
    fn a_whole_stream_is_walked_end_to_end() {
        let data = stream(5);
        let structure = Structure::of(&data).expect("it parses");

        assert!(structure.is_whole(data.len()));
        assert_eq!(structure.packets.len(), 5 * 4 + 1);
        assert_eq!(structure.last_complete_end, data.len());
    }

    /// The stuffing count lives in the low bits of the pack header's last byte, so a reader that
    /// ignores it lands inside the stuffing and the whole chain after it is lost.
    #[test]
    fn pack_stuffing_is_counted() {
        for stuffing in 0..8 {
            let pack = mpeg2_pack(stuffing);
            let packet = read_packet(&pack, 0).expect("it parses");
            assert_eq!(packet.len, MPEG2_PACK_LEN + stuffing);
        }
    }

    #[test]
    fn an_mpeg_1_pack_header_is_shorter_and_has_no_stuffing() {
        let mut pack = vec![0x00, 0x00, 0x01, PACK_HEADER, 0x21];
        pack.resize(MPEG1_PACK_LEN, 0);
        assert_eq!(read_packet(&pack, 0).expect("it parses").len, MPEG1_PACK_LEN);
    }

    /// The interrupted-recording case: the last packet promises bytes the file does not have.
    #[test]
    fn a_cut_stream_reports_where_the_last_whole_packet_ended() {
        let mut data = stream(4);
        let complete = Structure::of(&data).expect("parses").last_complete_end;
        data.extend_from_slice(&pes(0xE0, 4096)[..100]);

        let structure = Structure::of(&data).expect("parses");
        assert!(!structure.is_whole(data.len()));
        assert_eq!(structure.last_complete_end, complete);
        assert!(structure.packets.last().is_some_and(|packet| !packet.complete));
    }

    /// A carve starts at a sector boundary, which is rarely a packet boundary.
    #[test]
    fn a_stream_that_begins_mid_sector_is_still_found() {
        let mut data = vec![0x5A; 777];
        data.extend_from_slice(&stream(3));

        let structure = Structure::of(&data).expect("parses");
        assert_eq!(structure.leading_junk, 777);
        assert!(!structure.is_whole(data.len()), "the junk has to be visible");
        assert_eq!(structure.kept(), 777..data.len());
    }

    #[test]
    fn a_file_that_is_not_a_program_stream_is_not_claimed() {
        assert!(Structure::of(&[]).is_none());
        assert!(Structure::of(&vec![0u8; 4096]).is_none());
        // A lone start code with no chain behind it.
        assert!(Structure::of(&mpeg2_pack(0)).is_none());
    }

    /// A raw elementary stream opens with a sequence header, which carries no length; the walk
    /// must decline rather than invent one.
    #[test]
    fn an_elementary_stream_start_code_is_not_a_packet() {
        assert!(read_packet(&[0x00, 0x00, 0x01, 0xB3, 0, 0, 0, 0], 0).is_none());
        assert!(read_packet(&[0x00, 0x00, 0x01, 0x00, 0, 0, 0, 0], 0).is_none());
    }

    #[test]
    fn truncating_the_stream_anywhere_never_panics() {
        let data = stream(6);
        for len in 0..data.len() {
            let _ = Structure::of(&data[..len]);
        }
    }
}
