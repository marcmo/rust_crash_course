use bytebuffer::{ ByteBuffer };

const HEADER_LEN: usize = 16;

#[derive(Debug)]
pub struct MsgHeader {
    version: u8,
    msg_type: u8,
    reserved: u8,
    id_1: u8,
    id_2: u32,
    timestamp: u32,
    payload_length: u32,
}

impl MsgHeader {

    pub fn as_buffer(&mut self) -> ByteBuffer {
        let mut buffer: ByteBuffer = ByteBuffer::new();
        buffer.write_u8(self.version);
        buffer.write_u8(self.msg_type);
        buffer.write_u8(self.reserved);
        buffer.write_u8(self.id_1);
        buffer.write_u32(self.id_2);
        buffer.write_u32(self.timestamp);
        buffer.write_u32(self.payload_length);
        buffer
    }

}

#[derive(Debug)]
pub struct Msg {
    header: MsgHeader,
    payload: ByteBuffer,
}

#[derive(Debug)]
pub struct MsgBuffer {
    buffer: ByteBuffer,
}

impl Msg {

    pub fn as_bytes(&mut self) -> ByteBuffer {
        let mut buffer: ByteBuffer = ByteBuffer::from(self.header.as_buffer());
        buffer.write_bytes(&self.payload.read_bytes(self.payload.len()));
        buffer
    }

}

impl MsgBuffer {
    
    pub fn new(buffer: ByteBuffer) -> Self {
        MsgBuffer { buffer: buffer }
    }

    fn get_header(&mut self) -> Option<MsgHeader> {
        if (self.buffer.len() - self.buffer.get_rpos()) < HEADER_LEN {
            return None;
        }
        Some(MsgHeader {
            version: self.buffer.read_u8(),
            msg_type: self.buffer.read_u8(),
            reserved: self.buffer.read_u8(),
            id_1: self.buffer.read_u8(),
            id_2: self.buffer.read_u32(),
            timestamp: self.buffer.read_u32(),
            payload_length: self.buffer.read_u32(),
        })
    }

}

impl Iterator for MsgBuffer {
    
    type Item = Msg;

    fn next(&mut self) -> Option<Msg> {
        match self.get_header() {
            Some(header) => {
                let payload_length = header.payload_length as i64;
                let left: i64 = self.buffer.len() as i64 - self.buffer.get_rpos() as i64 - header.payload_length as i64;
                if left < 0 {
                    return None;
                }
                return Some(Msg {
                    header: header,
                    payload: ByteBuffer::from_bytes(&self.buffer.read_bytes(payload_length as usize)),
                });
            },
            None => {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use bytebuffer::{ ByteBuffer };
    use super::*;

    #[test]
    fn decode() {
        match File::open("./test/messages.bin") {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                let mut msgs: Vec<Msg> = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let mut msg_buffer = MsgBuffer::new(ByteBuffer::from_bytes(&buffer));
                        loop {
                            match msg_buffer.next() {
                                Some(msg) => {
                                    msgs.push(msg);
                                },
                                None => { break }
                            }
                        }
                        assert_eq!(msgs.len(), 8);
                    },
                    Err(_e) => assert_eq!(true, false),
                }
            },
            Err(_e) => assert_eq!(true, false),
        }
    }

    #[test]
    fn encode() {
        let mut msg: Msg = Msg {
            header: MsgHeader {
                version: 1,
                msg_type: 2,
                reserved: 3,
                id_1: 4,
                id_2: 5,
                timestamp: 6,
                payload_length: 10,
            },
            payload: ByteBuffer::from_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        };
        let buffer = msg.as_bytes();
        assert_eq!(buffer.len(), 26);
        let mut msg_buffer = MsgBuffer::new(buffer);
        match msg_buffer.next() {
            Some(m) => {
                assert_eq!(m.payload.len(), 10);
                assert_eq!(m.header.version, 1);
                assert_eq!(m.header.msg_type, 2);
                assert_eq!(m.header.reserved, 3);
                assert_eq!(m.header.id_1, 4);
                assert_eq!(m.header.id_2, 5);
                assert_eq!(m.header.timestamp, 6);
            },
            None => assert_eq!(true, false),
        }
    }
}
