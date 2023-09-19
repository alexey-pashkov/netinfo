use windows::Win32::NetworkManagement::IpHelper::{IP_ADAPTER_INFO, IP_ADDR_STRING};
use std::borrow::Cow;
use core::str;
use std::fmt::Display;

#[derive(Debug)]
pub struct DevInfo{
    pub dev_name: String,
    mac_addr: String,
    ip_addr: String
}

impl DevInfo{
    pub fn from_adapter_info(adapter: *mut IP_ADAPTER_INFO) -> Result<Self, &'static str>{
        if !adapter.is_null() {
            let adapter = unsafe {*adapter};
            let adp_name = u8_arr_to_string(adapter.Description.as_slice())?;
            let ip_addr = parse_ip_addr_list(&adapter.IpAddressList)?;
            let mac_adr = parse_mac(adapter.Address.as_slice());
            Ok(Self { dev_name: adp_name, mac_addr: mac_adr, ip_addr})
        }
        else {
            Err("Adapter is null!")
        }
    }
}

impl Display for DevInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Adapter name: {adp_nm}\nMAC-address: {mac}\nIP-address: {ip}\n", adp_nm = self.dev_name, mac = self.mac_addr, ip = self.ip_addr)
    }
}

fn u8_arr_to_string(data: &[u8]) -> Result<String, &'static str>{
    match String::from_utf8_lossy(data){
        Cow::Borrowed(string) => Ok(string.trim_matches('\0').to_owned()),
        Cow::Owned(_) => Err("This string is not valid UTF-8 string!")
    }
}

fn parse_ip_addr_list(ip_addr_list: &IP_ADDR_STRING) -> Result<String, &'static str>{
    match u8_arr_to_string(ip_addr_list.IpAddress.String.as_slice()) {
        Ok(mut ip_adr_str) => {
            ip_adr_str = ip_adr_str.chars().take_while(|c| *c != '\0').collect();
            if ip_adr_str.as_str() == "0.0.0.0" {
                Ok(String::from("No IP-address"))
            }
            else {
                let mut next_addr = ip_addr_list.Next;
                while !next_addr.is_null(){
                    let ip_addr = u8_arr_to_string(unsafe {*next_addr}.IpAddress.String.as_slice())?;
                    next_addr = unsafe {*next_addr}.Next;
                    ip_adr_str.push_str(format!("\n{}", ip_addr).as_str())
                }

                Ok(ip_adr_str)
            }
        },
        Err(_) => Err("Unable to parse IP-address!")
    }
    
}

fn parse_mac(mac_bytes: &[u8]) -> String{
    let mac_bytes = &mac_bytes[..6];
    mac_bytes.into_iter().map(|byte| {
        format!("{byte:X}")    
    }).collect::<Vec<String>>().join("-")
}