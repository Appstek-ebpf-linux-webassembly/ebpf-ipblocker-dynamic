use anyhow::Context;
use aya::{
    include_bytes_aligned,
    maps::HashMap,
    programs::{Xdp, XdpFlags},
    Bpf,
};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use std::net::Ipv4Addr;
use serde_json::{json, Value};
use std::thread;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

use aya::maps::MapRefMut;
use std::iter::Map;
use std::ops::Deref;
use aya_log::BpfLogger;
use clap::Parser;
use log::{info, warn};

use tokio::signal;
mod iptool;
mod util;
//mod wacher;
#[derive(Debug, Deserialize, Serialize)]
    struct Policy {
        ip: String,
    }

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/xdp-drop"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/xdp-drop"
    ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut Xdp = bpf.program_mut("xdp").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::SKB_MODE)
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // 

    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("BLOCKLIST")?)?;
////////////////////////////////////////////////////////
let block_addr: u32 = Ipv4Addr::new(162,159,136,234).try_into()?;
blockip(& mut blocklist,block_addr);
asyncall(&mut blocklist);

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
/*fn blockip( map: &mut HashMap<MapRefMut, u32, u32> ) -> Result<(), anyhow::Error>{
    let block_addr: u32 = Ipv4Addr::new(192,168,0,2).try_into()?;
    map.insert(block_addr,0,0);   
    Ok(())

} */


fn blockip( map: &mut HashMap<MapRefMut, u32, u32>,block_addr : u32 ) -> Result<(), anyhow::Error>{
    map.insert(block_addr,0,0);   
    Ok(())

}  


fn asyncall(map: &mut HashMap<MapRefMut, u32, u32>){
    //let path="./";
let path="../../../";
    futures::executor::block_on(async {
        if let Err(e) = async_watch(path,map).await {
            print!("error: {:?}", e)
        }
    });
}
 
    pub fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);
    
        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        }, Config::default())?;
    
        Ok((watcher, rx))
    }
    
    pub async fn async_watch<P: AsRef<Path>>(path: P,map: &mut HashMap<MapRefMut, u32, u32>) -> notify::Result<()> {
        let (mut watcher, mut rx) = async_watcher()?;
    
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
         println!("Watcher called ") ;
        while let Some(res) = rx.next().await {
            match res {
                Ok(event) =>{ 
                    println!("changed: {:?}", event);
                    thread::sleep(Duration::from_millis(2000));
                    let data =util::readjson().unwrap();
                    //println!("{:?}", data);
                    let parsed_data: Value = serde_json::from_value(data).unwrap();
                        //println!("Parsed data: {:?}", parsed_data[0]["ip"]);
                    
                        if let Some(array) = parsed_data.as_array() {
                            // Iterate over the elements in the array
                            for element in array {
                                // Access the values in each element
                                let ip = element["ip"].as_str().unwrap();
                                
                    
                                // Do something with the values
                                //println!("ip: {}", ip);
                                let ip: Result<Ipv4Addr, _> = ip.parse();
    
                                match ip {
                                    Ok(ip_addr) => {
                                        // Successfully parsed the IP address
                                        println!("Parsed IP: {}", ip_addr);
                                      let octets = ip_addr.octets();
                                        println!("{:?}", octets);
                                     let ip= iptool::create_ipv4_addr(ip.unwrap()).unwrap();
                                        
                                        map.insert(ip , 0, 0);                
                                    }
                                    Err(_) => {
                                        // Failed to parse the IP address
                                        println!("Invalid IP address");
                                    }
                                }    
    
                            }
                        } else {
                            println!("Invalid JSON array");
                        }
                }                   
                ,
                Err(e) => println!("watch error: {:?}", e),
            }
            
        }
    
        Ok(())
    }
    


/*
use std::net::Ipv4Addr;

use aya::map::{HashMap, MapRefMut};

fn block_ip(map: &mut HashMap<MapRefMut, u32, u32>, address: Ip) {
    let address: u32 = address.into();
    map.insert(address, 0);
}

fn main() {
    [here goes your usual code to load and attach the program]

    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("BLOCKLIST")?)?;

    block_ip(&mut blocklist, Ipv4Addr::new(192, 168, 0, 100));
}
 */
/*
    let mut only = ||-> Result<(), anyhow::Error>{
        println!("From closure: ");
        let block_addr: u32 = Ipv4Addr::new(192,168,29,123).try_into()?;
        blocklist.insert(block_addr, 0, 0);

        Ok(())
    };

    only();
    */ 