// use confshoter::detector;
use confshoter::{generator::config_generator::write_json,detector::exec::OsInfo};
fn main() {
   // get the OS INFO struct
   //write the OS INFO TO JSON
   // util to paese it
   let os_info = OsInfo::new();

   match os_info {
    Ok(info) => {
      
    },
    _ => {}
   }

}
