# Notes

## Compiling on nixos, copy to ubuntu (File not found)

### Solution
`sudo patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 /usr/local/musrs/musrs`

