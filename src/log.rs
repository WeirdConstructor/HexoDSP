// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::cell::RefCell;
use ringbuf::{RingBuffer, Producer, Consumer};
use lazy_static::lazy_static;

use std::sync::{Arc, Mutex};

lazy_static! {
    static ref LOG_RECV: Arc<Mutex<LogReceiver>> = {
        Arc::new(Mutex::new(LogReceiver::new()))
    };
}

thread_local! {
    pub static LOG: RefCell<Option<Log>> = RefCell::new(None);
}

pub fn retrieve_log_messages<F: FnMut(&str, &str)>(f: F) {
    if let Ok(mut lr) = LOG_RECV.lock() {
        lr.retrieve_messages(f);
    }
}

#[inline]
pub fn init_thread_logger(name: &'static str) -> bool {
    if !LOG.with(|l| l.borrow().is_some()) {
        if let Ok(mut lr) = LOG_RECV.lock() {
            lr.spawn_global_logger(name);
            return true;
        }
    }

    false
}

pub fn log<F: Fn(&mut std::io::BufWriter<&mut [u8]>)>(f: F) {
    use std::borrow::BorrowMut;

    LOG.with(|l| {
        let mut lh = l.borrow_mut();
        if let Some(lh) = (*(*lh.borrow_mut())).as_mut() {
            lh.log(f);
        }
    });
}

const MAX_LOG_BUFFER : usize = 4096;

pub struct LogReceiver {
    consumers:  Vec<(&'static str, Consumer<u8>)>,
}

impl LogReceiver {
    pub fn new() -> Self {
        Self {
            consumers: vec![],
        }
    }

    pub fn retrieve_messages<F: FnMut(&str, &str)>(&mut self, mut f: F) {
        for (name, consumer) in self.consumers.iter_mut() {
            let mut buf = [0; 1024];
            let mut oi = 0;

            while let Some(byte) = consumer.pop() {
                if oi >= buf.len() || byte == 0xFF {
                    f(name, std::str::from_utf8(&buf[0..oi]).unwrap());
                    oi = 0;
                } else {
                    buf[oi] = byte;
                    oi += 1;
                }
            }
        }
    }

    pub fn spawn_logger(&mut self, name: &'static str) -> Log {
        let rb = RingBuffer::new(MAX_LOG_BUFFER);
        let (producer, con) = rb.split();

        self.consumers.push((name, con));
        Log {
            producer,
            buf: [0; 512],
        }
    }

    #[inline]
    pub fn spawn_global_logger(&mut self, name: &'static str) {
        let hdl = self.spawn_logger(name);
        LOG.with(move |f| {
            *f.borrow_mut() = Some(hdl);
        });
    }
}

pub struct Log {
    producer: Producer<u8>,
    buf:      [u8; 512],
}

impl Log {
    pub fn log_buf(&mut self, data: &[u8]) {
        self.producer.push_slice(data);
        let _ = self.producer.push(0xFF);
    }

    pub fn log<F: Fn(&mut std::io::BufWriter<&mut [u8]>)>(&mut self, f: F) {
        self.buf.fill(0xFF);

        let len = {
            let mut bw = std::io::BufWriter::new(&mut self.buf[..]);
            f(&mut bw);
            bw.buffer().len()
        };

        if len < (self.buf.len() - 1) {
            // include one 0xFF!
            self.producer.push_slice(&self.buf[0..len + 1]);
        } else {
            self.producer.push_slice(&self.buf[0..len]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_threaded_logger() {
        std::thread::spawn(|| {
            use std::io::Write;
            assert!(init_thread_logger("tstlog"));
            log(|w| write!(w, "Test Log{}!", 1).unwrap());
            log(|w| write!(w, "Test Log{}!", 2).unwrap());
        });

        let mut msgs = vec![];
        for _ in 0..100 {
            std::thread::sleep(
                std::time::Duration::from_millis(100));

            retrieve_log_messages(|name, s| msgs.push(name.to_string() + "/" + s));

            if msgs.len() > 1 {
                assert_eq!(msgs[0], "tstlog/Test Log1!");
                assert_eq!(msgs[1], "tstlog/Test Log2!");
                break;
            }
        };
    }
}
