// jkcoxson

use tokio::{io::AsyncWriteExt, net::TcpStream};

const SERVICE_NAME: &str = "12:34:56:78:90:AB@fe80::de52:85ff:fece:c422._apple-mobdev2._tcp";

pub async fn register_device(udid: &str, ip: &str) -> Result<(), std::io::Error> {
    // Create a TCP stream to the muxer
    let mut stream = TcpStream::connect("127.0.0.1:32498").await?;
    // Send the register packet
    stream
        .write_all(format!("1\n{}\n{}\n{}\n", udid, SERVICE_NAME, ip).as_bytes())
        .await?;

    Ok(())
}

pub async fn unregister_device(udid: &str) -> Result<(), std::io::Error> {
    // Create a TCP stream to the muxer
    let mut stream = TcpStream::connect("127.0.0.1:32498").await?;
    // Send the register packet
    stream
        .write_all(format!("0\n{}\nyour\nmom\n", udid).as_bytes())
        .await?;
    Ok(())
}

pub async fn _unregister_all_devices() -> Result<(), std::io::Error> {
    let udids = match rusty_libimobiledevice::libimobiledevice::get_udid_list() {
        Ok(udids) => udids,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error getting udids",
            ));
        }
    };
    // Create a TCP stream to the muxer
    let mut stream = TcpStream::connect("127.0.0.1:32498").await?;
    for i in udids {
        stream
            .write_all(format!("0\n{}\nyour\nmom\n", i).as_bytes())
            .await?;
    }
    Ok(())
}

pub async fn connect_device(udid: &str, ip: &str) -> bool {
    // Wait for 0.5 seconds to give the device time to unregister
    println!("Waiting for device to appear in muxer...");
    // Register the device
    register_device(udid, ip).await.ok();
    for _ in 0..20 {
        let udids = match rusty_libimobiledevice::libimobiledevice::get_udid_list() {
            Ok(udids) => udids,
            Err(_) => {
                return false;
            }
        };
        if udids.contains(&udid.to_string()) {
            return true;
        }
        // Wait for 1 seconds to give the device time to register
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
    false
}