# fname = 'original_metalong.exr'
fname = 'original_str_one_byte.exr'
with open(fname, 'rb') as sourcef:
    original = sourcef.read()

fname = 'original_str_two_byte.exr'
with open(fname, 'rb') as sourcef:
    result = sourcef.read()


to = 649
assert original[:to] == result[:to]
fb = 904
assert original[fb:] == result[fb+1:]

f, t = 654, 904
assert original[f:t] == result[f+1, t+1]
