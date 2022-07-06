use std::fs::File;

pub struct Device;

impl Device {
    //Attempts to open a file in read-only mode.
    pub fn read_only() {
        let f = File::open("/dev/net/tun").unwrap();
        println!("{:?}",f);
        //File { fd: 3, path: "/dev/net/tun", read: true, write: false }
    }
}