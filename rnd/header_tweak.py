"""
image:
https://raw.githubusercontent.com/AcademySoftwareFoundation/openexr-images/main/ScanLines/Blobbies.exr
"""

fname = 'Blobbies.exr'
with open(fname, 'rb') as sourcef:
    data = sourcef.read()

# WORKING: Rename "capDate"
data_2 = data[:8] + b'blahbla' + data[15:]

# WORKING: rename "capDate" and replace the value
data_3 = data[:8] + b'blahbla' + data[15:]
value = b'1234567890123456789'
data_3 = data_3[:23] + bytes([len(value), 0, 0, 0]) + value + data_3[46:]

# Change "owner" attr
value = b'Copyright 2004 Industrial Light & Magic'
data_4 = (
    data[:354]
    + b'aaaaa\x00string\x00'
    # + bytes([len(value), 0, 0, 0])
    + value
    + b'\x00'
    + data[410:]
)

# # Add new string attr
# value = b'xxxxxxxxxxxxxxxxxxx'
# data_4 = (
#     data[:410]
#     + b'customattr'
#     + bytes([0])
#     + b'string'
#     + bytes([0, len(value), 0, 0, 0])
#     + value
#     + bytes([0])
#     + data[410:]
# )

# # rename "capDate" and replace the value to something longer
# data_4 = data[:8] + b'blahbla' + data[15:]
# value = b'abcd'
# data_4 = data_4[:23] + bytes([len(value), 0, 0, 0]) + value + data_4[46:]




# # remove capDate
# data_3 = data[:8] + data[46:]


# meta0 = list(b'shutup')
# meta0 += [0]
# meta0 += list(b'string')
# meta0 += [0]
# value = list(b'heyho')
# meta0 += [len(value), 0, 0, 0] + value

# data2 = list(data2)

# before = data2[:46]
# after = data2[46:]
# data2 = before + meta0 + after

# print(meta0)


with open('result.exr', 'wb') as destf:
    destf.write(bytes(data_4))
