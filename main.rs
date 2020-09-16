use async_std::net::UdpSocket;

#[async_std::main]
async fn main() {
    println!("{}", create_WoL_packet("192.168.2.12".to_string(), "00-0C-29-0A-DD-52".to_string()).await);
}

async fn create_WoL_packet(ip: String, mac: String) -> bool {
    let mut magic_packet_buffer = vec![];
    
    //Adding the 6 FF's that are necesarry for the packet.
    for x in 0..6 {
        let f = match u8::from_str_radix("FF", 16) {
            Ok(f) => f,
            Err(_) => return false,
        };

        magic_packet_buffer.push(f);
    }

    //Since the mac can have either a - (00-0E-81-0B-DD-51)
    if mac.contains("-") {
        let mac_splitter: Vec<&str> = mac.split("-").collect();

        for x in 0..16 {
            for mac_part in mac_splitter.iter(){
                let mac = match u8::from_str_radix(mac_part, 16) { //converting string to int
                    Ok(mac) => mac,
                    Err(_) => return false,
                };

                magic_packet_buffer.push(mac);
            }
        }
    
    //Since mac can also have a : (00:0E:81:0B:DD:51)
    } else if mac.contains(":") {
        let mac_splitter: Vec<&str> = mac.split(":").collect();

        for x in 0..16 {
            for mac_part in mac_splitter.iter(){
                let mac = match u8::from_str_radix(mac_part, 16) { //converting string to int
                    Ok(mac) => mac,
                    Err(_) => return false,
                };

                magic_packet_buffer.push(mac);
            }
        }
    } else {
        return false;
    }

    let res = send_packet(magic_packet_buffer, ip).await;
    if res == true {
        return true;
    } else {
        return false;
    }
}

async fn send_packet(magic_packet_buffer: Vec<u8>, target: String) -> bool {
    let mut socket = match UdpSocket::bind("0.0.0.0:0").await{
        Ok(socket) => socket,
        Err(_) => return false,
    };

    let mut send_target1 = match socket.send_to(&magic_packet_buffer, target).await {
        Ok(_) => return true,
        Err(e) => return false,
    };
}
