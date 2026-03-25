/*
    Copyright (C) 2026 GGroup and Gteam

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

// Implementing Local Service Discovery

use tokio::net::UdpSocket;
use std::net::{SocketAddr, Ipv6Addr};
use socket2::{Socket, Domain, Type, Protocol};
use std::error::Error;

const DISCOVERY_PORT: u16 = 9999;
const MULTICAST_ADDR_V6: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1);

pub async fn start_ipv6_discovery(my_identity: [u8; 32]) -> Result<(), Box<dyn Error>> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    #[cfg(not(windows))]
    socket.set_reuse_port(true)?;

    let address = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), DISCOVERY_PORT);
    socket.bind(&address.into())?;

    socket.join_multicast_v6(&MULTICAST_ADDR_V6, 0)?;

    let udp = UdpSocket::from_std(socket.into())?;
    println!("IPv6 Discovery run ff02::1:{}", DISCOVERY_PORT);

    let mut buf = [0u8; 120];

    loop { // TODO: "beacon" & "ship"
        tokio::select! {
            result = udp.recv_from(&mut buf) => {
                if let Ok((len, addr)) = result {
                    if len == 120 {
                        println!("{} {}", addr, len);
                    }
                }
            }

            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                let beacon = [0u8; 120];
                let target = SocketAddr::new(MULTICAST_ADDR_V6.into(), DISCOVERY_PORT);
                if let Err(e) = udp.send_to(&beacon, &target).await {
                    eprintln!("{}", e);
                }
            }
        }
    }
}

