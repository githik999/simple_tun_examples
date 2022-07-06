use std::fs::{File, OpenOptions};

use libc::O_RDWR;

pub struct Device;

impl Device {
    //Attempts to open a file in read-only mode.
    pub fn read_only() {
        let f = File::open("/dev/net/tun").unwrap();
        println!("{:?}",f);
        //File { fd: 3, path: "/dev/net/tun", read: true, write: false }
    }

    pub fn read_write() {
        let f = OpenOptions::new().read(true).write(true).open("/dev/net/tun").unwrap();
        println!("{:?}",f);
        //File { fd: 3, path: "/dev/net/tun", read: true, write: true }
    }

    pub fn libc() {
        let f = unsafe{ libc::open("/dev/net/tun".as_ptr() as *const _ ,O_RDWR); };
        println!("{:?}",f);
    }
}