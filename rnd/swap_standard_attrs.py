with open('../sample_file.exr', 'rb') as srcf:
    data = srcf.read()

data2 = (
    data[:8]

    + data[65:94]  # compression
    + data[171:196]  # line order
    + data[94:131]  # data window
    + data[131:171]  # display window
    + data[227:262]  # screen window center
    + data[196:227]  # pixel aspect
    + data[8:65]  # channel list
    + data[262:295]  # screen window width

    + data[295:]
)

with open('akslhdgf.exr', 'wb') as destf:
    destf.write(data2)
