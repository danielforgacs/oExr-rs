text = lambda x: list(x.encode('ascii'))

magic = [0x76, 0x2f, 0x31, 0x01]
version = [0x02, 0x00, 0x00, 0x00]
data = bytes(
    magic
    + version
    + text('cannels')
)
with open('sample.exr', 'wb') as exrf:
    exrf.write(data)
