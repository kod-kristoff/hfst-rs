with open('foma_header.bin', 'bw') as fp:
    fp.write(b"HFST\0")
    fp.write(b"\0\x1c\0")
    fp.write(b"version\0")
    fp.write(b"3.0\0")
    fp.write(b"type\0")
    fp.write(b"FOMA\0")
    fp.write(b"name\0")
    fp.write(b"\0")

