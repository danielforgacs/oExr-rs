with open('sample_file.exr', 'rb') as ff:
    data = ff.read()

scan_0_r_offset = 319 + 4 + 4
ch_len = 4

# rewriting first G pixel value
data = (
    data[:319 + 4 + 4]
    + bytes([0x03, 0x3F])
    + data[319 + 4 + 4 + 2:]
)

with open('rewrite.exr', 'wb') as ff:
    ff.write(data)
