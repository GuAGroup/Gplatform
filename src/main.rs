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

mod core;
use crate::core::beacon::beacon::*;

fn main() {
    init_seq();

    let signing_key = generate_identity();
    let node_id = signing_key.verifying_key().to_bytes();

    let magic = gen_magic();
    let seq = get_next_seq();
    let ts = get_timestamp();

    println!("--- Beacon Debug ---");
    println!("Magic (Hex):     0x{:08x}", magic);
    println!("Version:         1");

    print!("Node ID:         ");
    for byte in node_id {
        print!("{:02x}", byte);
    }
    println!();

    println!("Sequence Number: {}", seq);
    println!("Timestamp:       {}", ts);

    let mut data_to_sign = Vec::new();
    data_to_sign.extend_from_slice(&magic.to_le_bytes());
    data_to_sign.extend_from_slice(&seq.to_le_bytes());

    let signature = sign_packet(&signing_key, &data_to_sign);

    for i in 0..3 {
        let seq = get_next_seq();
        let ts = get_timestamp();
        println!("File #{}: Seq = {}, TS = {}", i, seq, ts);
    }

    println!("Signature: {:?}", signature);
    println!("--------------------------------");
}