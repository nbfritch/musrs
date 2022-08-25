Small proof of concept for audio streaming by using `partial-content` requests.

This wraps net/http for use in a gin app since gin does not support partial-content requests. By itself, gin will have requests that remain open waiting for the client to read more bytes from the file. `net/http` supports seeking and retreiving only the requested bytes from the requested file.