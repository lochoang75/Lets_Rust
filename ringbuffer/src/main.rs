#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum RingError {
    Full,
}

pub struct RingBuffer<const N: usize> {
    buf: [u8; N],
    head: usize,
    tail: usize,
    full: bool,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        assert!(N > 0);

        Self {
            buf: [0; N],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.head == self.tail && !self.full {
            return true;
        }

        return false;
    }

    pub fn is_full(&self) -> bool {
        return self.full;
    }

    pub fn capacity(&self) -> usize {
        return N
    }

    pub fn available(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        
        if self.is_full() {
            return N;
        }

        if self.head > self.tail {
            return self.head - self.tail;
        }

        return N - self.head + self.tail;
    }

    pub fn free_space(&self) -> usize {
        if self.is_empty() {
            return N;
        }
        
        if self.is_full() {
            return 0;
        }

        if self.head > self.tail {
            return N - (self.head - self.tail);
        }

        return self.tail - self.head;
    }

    /// Push data into buffer.
    /// Return number of bytes written or Err if full.
    pub fn push(&mut self, data: &[u8]) -> Result<usize, RingError> {
        if self.full {
            return Err(RingError::Full);
        }
        print!("Array before push: ");
        for num in self.buf {
            print!("{} ", num);
        }
        println!("");

        let mut byte_write: usize = 0;

        for byte_data in data {
            self.buf[self.head] = *byte_data;
            byte_write += 1;
            self.head += 1;

            if self.head == N {
                if  self.tail == 0 {
                    self.full = true;
                    self.head = 0;
                    break;
                }
                self.head = 0;
            } else if self.head == self.tail {
                self.full = true;
                break;
            }
        }

        print!("Array after push: ");
        for num in self.buf {
            print!("{} ", num);
        }
        println!("");
        
        return Ok(byte_write);
    }

    /// Pop data into `out`, return number of bytes read
    pub fn pop(&mut self, out: &mut [u8]) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut byte_pop: usize = 0;

        for _ in self.buf {
            out[byte_pop] = self.buf[self.tail];
            self.tail += 1;
            byte_pop += 1;
            self.full = false;
            if self.tail == N {
                self.tail = 0;
            }
            
            if byte_pop == out.len() {
                break;
            }
        }

        return byte_pop;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty() {
        let rb: RingBuffer<8> = RingBuffer::new();
        assert!(rb.is_empty());
        assert!(!rb.is_full());
        assert_eq!(rb.available(), 0);
        assert_eq!(rb.free_space(), 8);
    }

    #[test]
    fn push_and_pop_basic() {
        let mut rb: RingBuffer<8> = RingBuffer::new();

        let written = rb.push(b"abc").unwrap();
        assert_eq!(written, 3);
        assert_eq!(rb.available(), 3);

        let mut out = [0u8; 3];
        let read = rb.pop(&mut out);

        assert_eq!(read, 3);
        assert_eq!(&out, b"abc");
        assert!(rb.is_empty());
    }

    #[test]
    fn wrap_around_behavior() {
        let mut rb: RingBuffer<5> = RingBuffer::new();

        rb.push(b"abcd").unwrap();

        let mut tmp = [0u8; 2];
        rb.pop(&mut tmp);
        assert_eq!(&tmp, b"ab");

        rb.push(b"ef").unwrap();

        let mut out = [0u8; 4];
        let n = rb.pop(&mut out);

        assert_eq!(n, 4);
    }

    #[test]
    fn push_zero_bytes() {
        let mut rb: RingBuffer<8> = RingBuffer::new();
    
        let written = rb.push(b"").unwrap();
        assert_eq!(written, 0);
        assert!(rb.is_empty());
    }
    
    #[test]
    fn pop_from_empty_buffer() {
        let mut rb: RingBuffer<8> = RingBuffer::new();
    
        let mut out = [0u8; 4];
        let n = rb.pop(&mut out);
    
        assert_eq!(n, 0);
        assert!(rb.is_empty());
    }
    
    #[test]
    fn push_exact_capacity_then_pop_all() {
        let mut rb: RingBuffer<4> = RingBuffer::new();
    
        rb.push(b"abcd").unwrap();
        assert!(rb.is_full());
        assert_eq!(rb.available(), 4);
        assert_eq!(rb.free_space(), 0);
    
        let mut out = [0u8; 4];
        let n = rb.pop(&mut out);
    
        assert_eq!(n, 4);
        assert_eq!(&out, b"abcd");
        assert!(rb.is_empty());
    }
    
    #[test]
    fn push_more_than_capacity_partial_write() {
        let mut rb: RingBuffer<4> = RingBuffer::new();
    
        let written = rb.push(b"abcdef").unwrap();
        assert_eq!(written, 4);
        assert!(rb.is_full());
    
        let mut out = [0u8; 4];
        rb.pop(&mut out);
        assert_eq!(&out, b"abcd");
    }
    
    #[test]
    fn push_after_pop_from_full() {
        let mut rb: RingBuffer<4> = RingBuffer::new();
    
        rb.push(b"abcd").unwrap();
        assert!(rb.is_full());
    
        let mut tmp = [0u8; 2];
        rb.pop(&mut tmp);
        assert_eq!(&tmp, b"ab");
        assert!(!rb.is_full());
    
        rb.push(b"ef").unwrap();
    
        let mut out = [0u8; 4];
        let n = rb.pop(&mut out);
    
        assert_eq!(n, 4);
        assert_eq!(&out, b"cdef");
    }
    
    #[test]
    fn pop_less_than_available() {
        let mut rb: RingBuffer<8> = RingBuffer::new();
    
        rb.push(b"abcdef").unwrap();
    
        let mut out = [0u8; 3];
        let n = rb.pop(&mut out);
    
        assert_eq!(n, 3);
        assert_eq!(&out, b"abc");
        assert_eq!(rb.available(), 3);
    }
    
    #[test]
    fn multiple_small_pushes() {
        let mut rb: RingBuffer<8> = RingBuffer::new();
    
        rb.push(b"a").unwrap();
        rb.push(b"b").unwrap();
        rb.push(b"c").unwrap();
    
        let mut out = [0u8; 3];
        rb.pop(&mut out);
    
        assert_eq!(&out, b"abc");
    }
    
    #[test]
    fn head_tail_meet_without_full() {
        let mut rb: RingBuffer<4> = RingBuffer::new();
    
        rb.push(b"ab").unwrap();
    
        let mut out = [0u8; 2];
        rb.pop(&mut out);
    
        assert!(rb.is_empty());
        assert!(!rb.is_full());
    }
    
    #[test]
    fn alternating_push_pop() {
        let mut rb: RingBuffer<3> = RingBuffer::new();
    
        rb.push(b"a").unwrap();
        let mut out = [0u8; 1];
        rb.pop(&mut out);
        assert_eq!(&out, b"a");
    
        rb.push(b"b").unwrap();
        rb.push(b"c").unwrap();
    
        let mut out2 = [0u8; 2];
        rb.pop(&mut out2);
        assert_eq!(&out2, b"bc");
    }
    
    #[test]
    fn free_space_matches_available() {
        let mut rb: RingBuffer<10> = RingBuffer::new();
    
        rb.push(b"1234").unwrap();
        assert_eq!(rb.available() + rb.free_space(), rb.capacity());
    
        let mut out = [0u8; 2];
        rb.pop(&mut out);
    
        assert_eq!(rb.available() + rb.free_space(), rb.capacity());
    }
    
}
