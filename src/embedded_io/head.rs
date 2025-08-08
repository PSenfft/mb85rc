use embedded_io::SeekFrom;

/// moveable Read/Write head
/// Capped at `N`
/// Does not allow overflow
#[derive(Clone, Copy, Default)]
pub struct Head<const N: u64>(u64);

impl<const N: u64> Head<N> {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Option<u64> {
        match pos {
            SeekFrom::Start(offset) => match self.0.checked_add(offset).filter(|&sum| sum <= N) {
                Some(new_pos) => {
                    self.0 = new_pos;
                    Some(new_pos)
                }
                None => None,
            },
            SeekFrom::End(offset) => {
                // Do not allow seek over the end
                if offset > 0 {
                    return None;
                }

                self.0 = N - offset.unsigned_abs();

                Some(self.0)
            }
            SeekFrom::Current(offset) => {
                let new_pos = if offset > 0 {
                    self.0.checked_add(offset.unsigned_abs())
                } else {
                    self.0.checked_sub(offset.unsigned_abs())
                };

                match new_pos {
                    Some(pos) => {
                        if pos > N {
                            None
                        } else {
                            self.0 = pos;
                            Some(self.0)
                        }
                    }
                    None => None,
                }
            }
        }
    }

    /// Move the head forward when reading files.
    /// Expected to not overflow. If it does it is capped at `N`.
    pub fn advance(&mut self, bytes: usize) {
        match self.0.checked_add(bytes as u64) {
            Some(sum) => {
                if sum > N {
                    self.0 = N
                } else {
                    self.0 = sum;
                }
            }
            None => self.0 = N,
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

impl<const N: u64> From<Head<N>> for u64 {
    fn from(head: Head<N>) -> Self {
        head.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let head: Head<65_535> = Head::new();
        let inner: u64 = head.into();
        assert_eq!(inner, 0u64);
    }

    #[test]
    fn seek_start() {
        let mut head: Head<65_535> = Head::new();
        let res = head.seek(SeekFrom::Start(1337));

        let inner: u64 = head.into();
        assert_eq!(inner, 1337u64);
        assert_eq!(res, Some(inner));
    }

    #[test]
    fn seek_current_forward() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(3));

        let inner: u64 = head.into();
        assert_eq!(inner, 1340u64);
        assert_eq!(res, Some(inner));
    }

    #[test]
    fn seek_current_back() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(-337));

        let inner: u64 = head.into();
        assert_eq!(inner, 1000u64);
        assert_eq!(res, Some(inner));
    }

    #[test]
    fn seek_current_zero() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(1337));

        let res = head.seek(SeekFrom::Current(0));

        let inner: u64 = head.into();
        assert_eq!(inner, 1337u64);
        assert_eq!(res, Some(inner));
    }

    #[test]
    fn seek_end() {
        let mut head: Head<65_535> = Head::new();
        let res = head.seek(SeekFrom::End(-10));

        let inner: u64 = head.into();
        assert_eq!(inner, 65_535 - 10);
        assert_eq!(res, Some(inner));
    }

    #[test]
    fn seek_end_overflow() {
        let mut head: Head<65_535> = Head::new();
        let res = head.seek(SeekFrom::End(10));
        assert!(res.is_none());
    }

    #[test]
    fn seek_invalid_overflow() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(65_535 - 5));

        let res = head.seek(SeekFrom::Current(10));
        assert!(res.is_none());
    }

    #[test]
    fn seek_invalid_underflow() {
        let mut head: Head<65_535> = Head::new();

        let res = head.seek(SeekFrom::Current(-1));
        assert!(res.is_none());
    }

    #[test]
    fn convert_zero() {
        let head: Head<65_535> = Head::new();
        let addr = head.memory_address();

        assert_eq!(addr, Some([0, 0]));
    }

    #[test]
    fn convert_1_byte() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(50));

        let addr = head.memory_address();

        assert_eq!(addr, Some([0, 50]));
    }

    #[test]
    fn convert_2_bytes() {
        let mut head: Head<65_535> = Head::new();
        let _ = head.seek(SeekFrom::Start(260));

        let addr = head.memory_address();

        assert_eq!(addr, Some([1, 4]));
    }

    #[test]
    fn convert_invalid() {
        let mut head: Head<4_294_967_295> = Head::new(); // Needs to be more then u16::MAX 
        let _ = head.seek(SeekFrom::Start(65_536)); // One more then u16::MAX

        let addr = head.memory_address();
        assert!(addr.is_none());
    }
}
