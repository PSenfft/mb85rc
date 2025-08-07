use embedded_io::SeekFrom;

/// moveable Read/Write head
#[derive(Clone, Copy, Default)]
pub struct Head(u64);

impl Head {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, ()> {
        match pos {
            SeekFrom::Start(offset) => {
                self.0 = offset;
                Ok(self.0)
            }
            SeekFrom::End(offset) => {
                if offset > 0 {
                    return Err(());
                }

                self.0 = u64::MAX - offset.unsigned_abs();

                Ok(self.0)
            }
            SeekFrom::Current(offset) => {
                let new_pos = if offset > 0 {
                    self.0.checked_add(offset.unsigned_abs())
                } else {
                    self.0.checked_sub(offset.unsigned_abs())
                };

                match new_pos {
                    Some(pos) => {
                        self.0 = pos;
                        Ok(self.0)
                    }
                    None => Err(()),
                }
            }
        }
    }

    /// Move the head forward when reading files.
    /// Expected to not overflow. If it does it is capped at `u64::MAX`.
    pub fn advance(&mut self, bytes: usize) {
        match u64::try_from(bytes) {
            Ok(offset) => self.0 += offset,
            Err(_) => self.0 = u64::MAX,
        }
    }

    /// Convert to a 2 byte memory address used by the i2c interface.
    pub fn memory_address(&self) -> Option<[u8; 2]> {
        if self.0 > u16::MAX as u64 {
            return None;
        }

        let addr16 = self.0 as u16;
        let high = (addr16 >> 8) as u8;
        let low = (addr16 & 0xFF) as u8;

        Some([high, low])
    }
}

impl From<Head> for u64 {
    fn from(head: Head) -> Self {
        head.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let head = Head::new();
        let inner: u64 = head.into();
        assert_eq!(inner, 0u64);
    }

    #[test]
    fn seek_start() {
        let mut head = Head::new();
        let res = head.seek(SeekFrom::Start(1337));

        let inner: u64 = head.into();
        assert_eq!(inner, 1337u64);
        assert_eq!(res, Ok(inner));
    }

    #[test]
    fn seek_current_forward() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(3));

        let inner: u64 = head.into();
        assert_eq!(inner, 1340u64);
        assert_eq!(res, Ok(inner));
    }

    #[test]
    fn seek_current_back() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(-337));

        let inner: u64 = head.into();
        assert_eq!(inner, 1000u64);
        assert_eq!(res, Ok(inner));
    }

    #[test]
    fn seek_current_zero() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(0));

        let inner: u64 = head.into();
        assert_eq!(inner, 1337u64);
        assert_eq!(res, Ok(inner));
    }

    #[test]
    fn seek_end() {
        let mut head = Head::new();
        let res = head.seek(SeekFrom::End(-10));

        let inner: u64 = head.into();
        assert_eq!(inner, u64::MAX - 10);
        assert_eq!(res, Ok(inner));
    }

    #[test]
    fn seek_invalid_overflow() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(u64::MAX - 5));

        let res = head.seek(SeekFrom::Current(10));
        assert!(res.is_err());
    }

    #[test]
    fn seek_invalid_underflow() {
        let mut head = Head::new();

        let res = head.seek(SeekFrom::Current(-1));
        assert!(res.is_err());
    }

    #[test]
    fn convert_zero() {
        let head = Head::new();
        let addr = head.memory_address();

        assert_eq!(addr, Some([0, 0]));
    }

    #[test]
    fn convert_1_byte() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(50));

        let addr = head.memory_address();

        assert_eq!(addr, Some([0, 50]));
    }

    #[test]
    fn convert_2_bytes() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(260));

        let addr = head.memory_address();

        assert_eq!(addr, Some([1, 4]));
    }

    #[test]
    fn convert_invalid() {
        let mut head = Head::new();
        let _ = head.seek(SeekFrom::Start(u16::MAX as u64 + 1));

        let addr = head.memory_address();
        assert!(addr.is_none());
    }
}
