Here is what's new:
* Function ```sysifo.virtualization``` returning the name of the virtual environment under which BUND is running or Unknown
* Function ```sysifo.virtualization?``` returning TRUE or FALSE if BUND is running under virtual environment
* Functions ```sysinfo.mem.[total|free|used|shared|cached|buffers][|.]``` that is returning information about allocated memory for specific purpose in bytes.
* Functions ```encode.base[|.]``` that is encoding object on stack or workbench to BASE64
* Functions ```decode.base[|.]``` that is decoding object on stack or workbench from BASE64 into Value
* Fountion ```stat.count[|.][|,]``` returning the number of values in the data sample allocated in list, stack or metrics. Be careful, for stack it is a destructive operation
