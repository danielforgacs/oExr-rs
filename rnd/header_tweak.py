"""
image:
https://raw.githubusercontent.com/AcademySoftwareFoundation/openexr-images/main/ScanLines/Blobbies.exr
"""

fname = 'Blobbies.exr'
with open(fname, 'rb') as sourcef:
    data = sourcef.read()

attrname = b'blahbla'

data2 = data[:8]
data2 += attrname
data2 += data[15:]

meta0 = list(b'shutup')
meta0 += [0]
meta0 += list(b'string')
meta0 += [0]
value = list(b'heyho')
meta0 += [len(value), 0, 0, 0] + value

data2 = list(data2)

before = data2[:46]
after = data2[46:]
data2 = before + meta0 + after

print(meta0)


with open('result.exr', 'wb') as destf:
    destf.write(bytes(data2))
