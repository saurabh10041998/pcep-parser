[![build](https://github.com/saurabh10041998/pcep-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/saurabh10041998/pcep-parser/actions/workflows/rust.yml) [![Check and Lint](https://github.com/saurabh10041998/pcep-parser/actions/workflows/check-and-lint.yml/badge.svg)](https://github.com/saurabh10041998/pcep-parser/actions/workflows/check-and-lint.yml)  
# pcep-parser
Crate aiming for making parsing of PCEP protocol messages.

## message type supported
- [x] OPEN message
- [x] Keepalive message
- [ ] PCUpdate message
- [ ] PCInitiate message. 

## todo
- [x] Fix cicd errors due to TLV adding in open message.
- [ ] Fix clippy errors.
- [x] LSP Objects and it's Tlvs
- [ ] Path Object and it's Tlvs
- [ ] path attribute list
- [x] LSPA object. (Done except TLVs)
- [x] BANDWIDTH object
- [ ] METRIC object
- [ ] IRO object. 
- [ ] PcUpdate message
