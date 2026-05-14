use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct IvfParser {
    file: File,
    pub width: u16,
    pub height: u16,
    pub timebase_num: u32,
    pub timebase_den: u32,
    pub frame_cnt: u32,
}

pub struct IvfFrame {
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

impl IvfParser {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 32];
        file.read_exact(&mut header)?;

        if &header[0..4] != b"DKIF" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid IVF header"));
        }

        let width = u16::from_le_bytes(header[12..14].try_into().unwrap());
        let height = u16::from_le_bytes(header[14..16].try_into().unwrap());
        let timebase_den = u32::from_le_bytes(header[16..20].try_into().unwrap());
        let timebase_num = u32::from_le_bytes(header[20..24].try_into().unwrap());
        let frame_cnt = u32::from_le_bytes(header[24..28].try_into().unwrap());

        Ok(Self {
            file,
            width,
            height,
            timebase_num,
            timebase_den,
            frame_cnt,
        })
    }

    pub fn next_frame(&mut self) -> io::Result<Option<IvfFrame>> {
        let mut frame_header = [0u8; 12];
        match self.file.read_exact(&mut frame_header) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }

        let size = u32::from_le_bytes(frame_header[0..4].try_into().unwrap());
        let timestamp = u64::from_le_bytes(frame_header[4..12].try_into().unwrap());

        let mut payload = vec![0u8; size as usize];
        self.file.read_exact(&mut payload)?;

        Ok(Some(IvfFrame { payload, timestamp }))
    }
}
