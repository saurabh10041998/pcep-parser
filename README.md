[![build](https://github.com/saurabh10041998/pcep-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/saurabh10041998/pcep-parser/actions/workflows/rust.yml) [![Check and Lint](https://github.com/saurabh10041998/pcep-parser/actions/workflows/check-and-lint.yml/badge.svg)](https://github.com/saurabh10041998/pcep-parser/actions/workflows/check-and-lint.yml) ![release](https://img.shields.io/github/v/release/saurabh10041998/pcep-parser) 

# 🦀 pcep-parser 🦀 
Rust Crate aiming for the parsing of pcep messages

# How to compile

Prerequites:
- rustc :  1.66+
- cargo :  1.66+

```bash
git clone https://github.com/saurabh10041998/pcep-parser.git
cd pcep-parser
cargo build --release
cd target/release
```
You will find the release binary. 
Alternatively you can download the zip compactible to your operating system from `release` section.

## How to use it
Following python3 script will show how you can decode packet..

**driver.py**
```python
#! /usr/bin/env python3
import argparse
import os
from scapy.all import *

parser = argparse.ArgumentParser()
parser.add_argument("--file",required = True, help = "name of pcap file containing pcep packets..")

args = parser.parse_args()
opts = vars(args)

packets = rdpcap(opts["file"])

## loop to each packet of pcaps
for packet in packets:
    try:
        ## If packet has TCP layer
        if packet.haslayer(TCP):
            ## Get the TCP payload or PCEP packet
            payload = packet[TCP].payload
            ## If no packet, continue looping
            if len(payload.__bytes__()) == 0:
                continue
            ## Dump PCEP packet to file
            with open("tmp_packet", "wb") as f:
                f.write(payload.__bytes__())
            ## Call the binary to parse the dumped pcep packet
            os.system("pcep-parser/target/release/pcep-parser")
    finally:
        ## Delete the file when parsing is done..
        os.remove("tmp_packet")
```

To run it:
```bash
./driver.py --file pcep_packets.pcap
```

**Note** : The command line argument support soon be added to eliminated the need of such external scripts. 

## message type supported
- [x] OPEN message
- [x] Keepalive message
- [x] PCUpdate message. (Please see the following table matrix for what subobjects and Tlvs are supported).
- [x] PCInitiate message. (Please see the following table matrix for what subobjects and Tlvs are supported).
- [ ] PCRpt message.

More message, objects, subobjects and tlvs soon to be added in next release.

## PCEP Messages
|RFCs| Message Type Supported |
|----|----|
|[RFC5440](https://datatracker.ietf.org/doc/html/rfc5440)| Open, KeepAlive|
|[RFC8231](https://datatracker.ietf.org/doc/html/rfc8231) | PCUpdate|
|[RFC8281](https://datatracker.ietf.org/doc/html/rfc8281) | PCInitiate |


## PCEP Objects
|Object| RFCs | Supported TLVs |
|-----|-----|-----|
|OPEN| [RFC5440](https://datatracker.ietf.org/doc/html/rfc5440) | STATEFUL-PCE-CAPABILITY TLV, SR-PCE-CAPABILITY TLV |
|ENDPOINTS | [RFC5440](https://datatracker.ietf.org/doc/html/rfc5440) | IPv4Addresses supported |
|ERO| [RFC5440](https://datatracker.ietf.org/doc/html/rfc5440) | SR subobject [RFC8664](https://datatracker.ietf.org/doc/html/rfc8664), Ipv4Prefix Subobject [RFC3209](https://datatracker.ietf.org/doc/html/rfc3209) |
|SRP| [RFC8231](https://datatracker.ietf.org/doc/html/rfc8231)| No TLV supported yet | 
|LSP| [RFC8231](https://datatracker.ietf.org/doc/html/rfc8231),[RFC8281](https://datatracker.ietf.org/doc/html/rfc8281) | IPV4LSPIDENTIFIERS-TLV, SYMBOLICPATHNAME-TLV |
|LSPA | [RFC5440](https://datatracker.ietf.org/doc/html/rfc5440) | No TLVs |
|METRIC|[RFC8231](https://datatracker.ietf.org/doc/html/rfc8231) | Igp, Te, Hopcount, Sid-Depth, PathDelay metric supported |
|BANDWIDTH|[RFC5440](https://datatracker.ietf.org/doc/html/rfc5440)| Requested Bandwidth Type and Bandwidth of an existing TE LSP for which a reoptimization is requested |  


## PCEP subobjects
|Subobject|RFCs|Supported TLVs|
|---|---|---|
| SR-ERO | [RFC8664](https://datatracker.ietf.org/doc/html/rfc8664) | NAIType: Ipv4 Adjacency NAIType supported |
| IPv4Pefix | [RFC3209](https://datatracker.ietf.org/doc/html/rfc3209) | |
