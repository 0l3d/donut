#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod crlib;
use crlib::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let panic_msg = b"PANIC occurred!\n\0";
        write(2, panic_msg.as_ptr(), panic_msg.len() - 1);
        quit(1);
    }
}

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 24;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

unsafe fn spinning_donut() {
    let mut r1: f64 = 0.0;
    let mut r2: f64 = 0.0;
    let mut screen: [u8; SCREEN_SIZE] = [b' '; SCREEN_SIZE];
    let mut zbuffer: [f64; SCREEN_SIZE] = [0.0; SCREEN_SIZE];
    
  
    printf(b"\x1b[2J\0".as_ptr() as PCChar);
    
    loop {
  
        for i in 0..SCREEN_SIZE {
            screen[i] = b' ';
            zbuffer[i] = 0.0;
        }
        

        let mut b = 0.0;
        while b < 6.28 {
            let mut a = 0.0;
            while a < 6.28 {
                let sin_a = sin(a);
                let cos_a = cos(a);
                let sin_b = sin(b);
                let cos_b = cos(b);
                let sin_r1 = sin(r1);
                let cos_r1 = cos(r1);
                let sin_r2 = sin(r2);
                let cos_r2 = cos(r2);
                
            
                let h = cos_b + 2.0;
                let depth = 1.0 / (sin_a * h * sin_r1 + sin_b * cos_r1 + 5.0);
                let temp = sin_a * h * cos_r1 - sin_b * sin_r1;
                
            
                let x = (40.0 + 30.0 * depth * (cos_a * h * cos_r2 - temp * sin_r2)) as i32;
                let y = (12.0 + 15.0 * depth * (cos_a * h * sin_r2 + temp * cos_r2)) as i32;
                
            
                let screen_idx = (x + SCREEN_WIDTH as i32 * y) as usize;
                
           
                let luminance = 8.0 * ((sin_b * sin_r1 - sin_a * cos_b * cos_r1) * cos_r2 
                                     - sin_a * cos_b * sin_r1 
                                     - sin_b * cos_r1 
                                     - cos_a * cos_b * sin_r2);
                
         
                if y > 0 && y < SCREEN_HEIGHT as i32 
                   && x > 0 && x < SCREEN_WIDTH as i32 
                   && screen_idx < SCREEN_SIZE 
                   && depth > zbuffer[screen_idx] {
                    
                    zbuffer[screen_idx] = depth;
                    
              
                    let chars = b".,-~:;=!*#$@";
                    let char_idx = if luminance > 0.0 {
                        if luminance as usize >= chars.len() {
                            chars.len() - 1
                        } else {
                            luminance as usize
                        }
                    } else {
                        0
                    };
                    
                    screen[screen_idx] = chars[char_idx];
                }
                
                a += 0.02;
            }
            b += 0.07;
        }
        
     
        printf(b"\x1b[H\0".as_ptr() as PCChar);
        
        for j in 0..SCREEN_SIZE {
            if j % SCREEN_WIDTH == 0 && j > 0 {
                printf(b"\n\0".as_ptr() as PCChar);
            }
            printf(b"%c\0".as_ptr() as PCChar, screen[j] as i32);
        }
        
   
        r1 += 0.04;
        r2 += 0.02;
        
   
        usleep(40000);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
     spinning_donut();
    

    quit(0);
}
