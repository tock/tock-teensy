#!env python

f = open("strip.bin", 'wb');
for i in range(0,128):
    f.write(b'\xff')
f.close()

