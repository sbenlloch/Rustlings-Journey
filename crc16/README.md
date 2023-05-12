# CRC16 Checksum Utility

## About CRC16

CRC16 is a hash function that generates a 16-bit checksum for data integrity verification. It's used in digital networks and storage devices.

More in [Wikipedia](https://en.wikipedia.org/wiki/Cyclic_redundancy_check)

## Utility Usage

This utility, provided in Rust and Python, calculates the CRC16 checksum of a file, enabling you to verify its integrity during transfer or storage.

### Rust

Compile and run the Rust program with the file path as a command-line argument:

```bash
$ rustc crc16.rs
$ ./crc16 file_to_check
```

### Python

Similarly, run the Python script with the file path as a command-line argument:

```bash
$ python crc16.py file_to_check
```

## Conclusion

With CRC16, you can ensure data integrity, detecting accidental alterations in data transmission/storage. 

**Note:** Be aware that while CRC16 is useful for error-checking, it does not provide security against intentional data tampering.

