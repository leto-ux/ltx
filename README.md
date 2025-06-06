# THIS IS A WIP, PROBABLY AWFUL AND DISFUNCTIONAL PIECE OF S***WARE

### Install

```bash
cargo build --release
```
The built binary is stored in: 'target/release/ltx'

### Usage

Every method is named the same as what is used in the [Litecoin Core](https://github.com/litecoin-project/litecoin)
project's RPC integration (whose documentation is just the [Bitcoin Core](https://github.com/bitcoin/bitcoin)
RPC, available [here](https://developer.bitcoin.org/reference/rpc/).

```bash
ltx --sendtoaddress <address> <amount>
ltx --getnewaddress <label (optional)>
ltx --getbalance <confirmation_count(optional)>
ltx --listaddressgroupings
ltx --help
```
