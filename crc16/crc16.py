import sys
import crcmod

def crc16_file(filename):
    try:
        with open(filename, 'rb') as f:
            buf = f.read()
    except IOError:
        print('Cannot open file: {}'.format(filename))
        sys.exit(1)

    crc16_func = crcmod.mkCrcFun(0x18005, rev=True, initCrc=0xFFFF, xorOut=0x0000)
    crc = crc16_func(buf)
    return crc

def main():
    if len(sys.argv) < 2:
        print("Please provide a file path")
        sys.exit(1)

    path = sys.argv[1]
    crc = crc16_file(path)
    print("CRC16 checksum: 0x{:X}".format(crc))

if __name__ == "__main__":
    main()
