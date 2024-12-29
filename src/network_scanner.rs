use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::{lookup_host, TcpStream};
use tokio::time::timeout;


#[derive(Debug, Clone)]
pub struct ScanResult {
    pub address: String,
    pub port: u16,
    pub open: bool,
}


pub async fn scan_target(target: &str) -> Vec<ScanResult> {
    
    let ip = match resolve_to_ip(target).await {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error resolving IP: {}", e);
            return Vec::new();
        }
    };

    
    let ports = [21, 22, 23, 25, 53, 80, 110, 143, 443, 3000, 5000, 8000, 8080];

    
    let mut results = Vec::new();
    for &port in &ports {
        let is_open = check_port(ip, port).await;
        results.push(ScanResult {
            address: target.to_string(),
            port,
            open: is_open,
        });
    }

    results
}


async fn resolve_to_ip(target: &str) -> Result<IpAddr, String> {
    let clean_target = target
        .strip_prefix("http://")
        .or_else(|| target.strip_prefix("https://"))
        .unwrap_or(target)
        .trim();

    
    if let Ok(ip) = clean_target.parse::<IpAddr>() {
        return Ok(ip);
    }

    
    let lookup_result = lookup_host(&format!("{}:80", clean_target))
        .await
        .map_err(|e| e.to_string())?
        .next()
        .ok_or_else(|| "No IP addresses found".to_string())?;

    Ok(lookup_result.ip())
}

async fn check_port(ip: IpAddr, port: u16) -> bool {
    let addr = SocketAddr::new(ip, port);
    matches!(
        timeout(Duration::from_secs(1), TcpStream::connect(addr)).await,
        Ok(Ok(_))
    )
}

#[tokio::main]
async fn main() {
    let target = "google.com"; // (isi yg mau di test) contoh google.com 
    println!("Scanning target: {}", target);

    let results = scan_target(target).await;
    
    println!("\nScan Results:");
    println!("-------------");
    for result in results {
        println!(
            "Port {:5} : {}",
            result.port,
            if result.open { "Open" } else { "Closed" }
        );
    }
}