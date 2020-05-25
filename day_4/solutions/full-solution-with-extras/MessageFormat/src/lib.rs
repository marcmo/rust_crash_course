use core::mem::size_of;
use nom::number::streaming::be_u32;
use nom::number::streaming::be_u8;
use nom::IResult;
use nom::*;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Error, ErrorKind, Read, Result};

#[derive(PartialEq, Debug, Eq)]
struct Header {
    version: u8,
    r#type: u8,
    reserved: u8,
    id1: u8,
    id2: u32,
    timestamp: u32,
    length: u32,
}

#[derive(PartialEq, Debug, Eq, Serialize, Deserialize)]
pub struct Message {
    version: u8,
    r#type: u8,
    reserved: u8,
    id1: u8,
    id2: u32,
    timestamp: u32,
    data: Vec<u8>,
}

impl Message {
    pub const HEADER_SIZE: usize = size_of::<Header>();

    fn new(header: Header, data: Vec<u8>) -> Self {
        assert_eq!(header.length as usize, data.len());
        Message {
            version: header.version,
            r#type: header.r#type,
            reserved: header.reserved,
            id1: header.id1,
            id2: header.id2,
            timestamp: header.timestamp,
            data,
        }
    }
}

pub struct MessageIterator<R> {
    reader: BufReader<R>,
}

impl<R: Read> MessageIterator<R> {
    #[allow(missing_docs)]
    pub fn new(reader: R) -> Self {
        MessageIterator {
            reader: BufReader::new(reader),
        }
    }
}

pub struct ByteView {
    message: Message,
    offset: usize,
}

impl ByteView {
    pub fn new(message: Message) -> Self {
        ByteView { message, offset: 0 }
    }
}

impl Iterator for ByteView {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.offset >= Message::HEADER_SIZE + self.message.data.len() {
            return None;
        }
        let byte: u8 = match self.offset {
            0 => self.message.version,
            1 => self.message.r#type,
            2 => self.message.reserved,
            3 => self.message.id1,
            4..=7 => {
                let index = self.offset - 4;
                self.message.id2.to_be_bytes()[index]
            }
            8..=11 => {
                let index = self.offset - 8;
                self.message.timestamp.to_be_bytes()[index]
            }
            12..=15 => {
                let index = self.offset - 12;
                let payload_size: u32 = self.message.data.len() as u32;
                payload_size.to_be_bytes()[index]
            }
            _ => {
                let index = self.offset - Message::HEADER_SIZE;
                self.message.data[index]
            }
        };
        self.offset += 1;
        Some(byte)
    }
}

impl<R: Read> Iterator for MessageIterator<R> {
    type Item = Result<Message>;

    fn next(&mut self) -> Option<Result<Message>> {
        let mut buf: [u8; Message::HEADER_SIZE] = [0; Message::HEADER_SIZE];
        match self.reader.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    None
                } else if n < Message::HEADER_SIZE {
                    Some(Err(Error::new(ErrorKind::Other, "Input data is corrupt")))
                } else {
                    match header(&buf) {
                        Ok((_i, header)) => {
                            let mut data: Vec<u8> = vec![0; header.length as usize];
                            match self.reader.read_exact(&mut data) {
                                Ok(_) => Some(Ok(Message::new(header, data))),
                                Err(_e) => Some(Err(Error::new(
                                    ErrorKind::Other,
                                    "Could not read payload",
                                ))),
                            }
                        }
                        Err(_e) => Some(Err(Error::new(ErrorKind::Other, "Parsing failed"))),
                    }
                }
            }
            Err(_e) => Some(Err(Error::new(
                ErrorKind::Other,
                "Could not read from input",
            ))),
        }
    }
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    do_parse!(
        input,
        v: be_u8
            >> t: be_u8
            >> r: be_u8
            >> id1: be_u8
            >> id2: be_u32
            >> timestamp: be_u32
            >> payload_length: be_u32
            >> (Header {
                version: v,
                r#type: t,
                reserved: r,
                id1: id1,
                id2: id2,
                timestamp: timestamp,
                length: payload_length
            })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn byteview_of_message() {
        let message = Message {
            version: 1,
            r#type: 2,
            reserved: 0,
            id1: 0x0A,
            id2: 0xffaaffaa,
            timestamp: 0xbbaabbaa,
            data: vec![0; 0],
        };
        let expected: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
            0xbb, 0xaa, 0xbb, 0xaa, // timestamp
            0x00, 0x00, 0x00, 0x00, // payload length
        ];
        let encoder = super::ByteView::new(message);

        assert_eq!(expected, encoder.into_iter().collect::<std::vec::Vec<u8>>());
    }

    #[test]
    fn message_iterator_contains_one_message_with_empty_payload() {
        let expected = Message {
            version: 1,
            r#type: 2,
            reserved: 0,
            id1: 0x0A,
            id2: 0xffaaffaa,
            timestamp: 0xbbaabbaa,
            data: vec![0; 0],
        };
        let input: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
            0xbb, 0xaa, 0xbb, 0xaa, // timestamp
            0x00, 0x00, 0x00, 0x00, // payload length
        ];
        let reader = Cursor::new(input);
        let mut iter = MessageIterator::new(reader);

        let result = iter.next().unwrap();
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message, expected);
    }

    #[test]
    fn message_iterator_contains_one_message_with_payload() {
        let expected = Message {
            version: 1,
            r#type: 2,
            reserved: 0,
            id1: 0x0A,
            id2: 0xffaaffaa,
            timestamp: 0xbbaabbaa,
            data: vec![1, 2, 3, 4],
        };
        let input: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
            0xbb, 0xaa, 0xbb, 0xaa, // timestamp
            0x00, 0x00, 0x00, 0x04, // payload length
            0x01, 0x02, 0x03, 0x04, // payload
        ];
        let reader = Cursor::new(input);
        let mut iter = MessageIterator::new(reader);

        let result = iter.next().unwrap();
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message, expected);
    }

    #[test]
    fn message_iterator_encounters_invalid_message() {
        let input: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
            0xbb, 0xaa, 0xbb, 0xaa, // timestamp
            0x00, 0x00, 0x00, 0x06, // payload length
            0x01, 0x02, 0x03, 0x04, // payload
        ];
        let reader = Cursor::new(input);
        let mut iter = MessageIterator::new(reader);

        let result = iter.next().unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn message_iterator_encounters_invalid_message_header() {
        let input: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
        ];
        let reader = Cursor::new(input);
        let mut iter = MessageIterator::new(reader);

        let result = iter.next().unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn parse_header() {
        let expected = IResult::Ok((
            &b""[..],
            Header {
                version: 1,
                r#type: 2,
                reserved: 0,
                id1: 10,
                id2: 0xffaaffaa,
                timestamp: 0xbbaabbaa,
                length: 0,
            },
        ));
        let input: Vec<u8> = vec![
            0x01, // version
            0x02, // type
            0x00, // reserved
            0x0A, // id1
            0xff, 0xaa, 0xff, 0xaa, // id2
            0xbb, 0xaa, 0xbb, 0xaa, // timestamp
            0x00, 0x00, 0x00, 0x00, // payload length
        ];

        assert_eq!(header(&input), expected);
    }

    #[test]
    fn new_message() {
        let expected = Message {
            version: 1,
            r#type: 2,
            reserved: 0,
            id1: 10,
            id2: 0xffaaffaa,
            timestamp: 0xbbaabbaa,
            data: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        };
        let header = Header {
            version: 1,
            r#type: 2,
            reserved: 0,
            id1: 10,
            id2: 0xffaaffaa,
            timestamp: 0xbbaabbaa,
            length: 5,
        };
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];

        assert_eq!(Message::new(header, data), expected);
    }
}
