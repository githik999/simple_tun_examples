/* 
Typically, an ioctl takes 3 parameters as arguments:


1.An open file descriptor, fd.


2.An device-dependennt request code or operation. 

This request code is referred to as op in this module.


3.Either a pointer to a location in memory or an integer. 

This number of pointer may either be used by the kernel or written to by the kernel 

depending on how the operation is documented to work.


*/

/* 

run by root

*/


pub struct Device;

impl Device {
    
}