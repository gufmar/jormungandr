# TX creator

Creates a signed transaction

## Building

```
cargo build --bin tx_create
```

## Usage

```
tx_create <options>
```

The options are

- -c <value> or --fee-constant <value> - fee which will be always added to the transaction
- -a <value> or --fee-per-addr <value> - fee which will be added to the transaction for every
input and output
- -i <input> or --input <input> - transaction input. Must have format
`<hex-encoded-transaction-id>:<output-index>:<value>`. E.g. `1234567890abcdef:2:535`.
At least 1 value required.
- -o <output> or --output <output> -transaction output. Must have format
`<hex-encoded-address>:<value>`. E.g. `abcdef1234567890:501`. At least 1 value required.
- -r <address> or --return <address> - return address. Value taken from inputs and not spent on
outputs or fees will be returned to this address. If not provided, the excess will go to treasury.
Must be hex-encoded.
- -s <key> or --spending-key <key> - transaction spending keys. Must be hex-encoded.
Required as many as provided inputs.

Value outputted to stdout on success is binary blob with transaction
