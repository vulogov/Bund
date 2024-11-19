Here is what's new:
* Function ```sysifo.virtualization``` returning the name of the virtual environment under which BUND is running or Unknown
* Function ```sysifo.virtualization?``` returning TRUE or FALSE if BUND is running under virtual environment
* Functions ```sysinfo.mem.[total|free|used|shared|cached|buffers][|.]``` that is returning information about allocated memory for specific purpose in bytes.
* Functions ```encode.base[|.]``` that is encoding object on stack or workbench to BASE64
* Functions ```decode.base[|.]``` that is decoding object on stack or workbench from BASE64 into Value
* Function ```stat.count[|.][|,]``` returning the number of values in the data sample allocated in list, stack or metrics. Be careful, for stack it is a destructive operation
* Function ```string.distance.levenshtein[|.]``` returning the distance between two strings using Levenshtein algorithm
* Function ```string.distance.damerau_levenshtein[|.]``` returning the distance between two strings using Damerau-Levenshtein algorithm
* Function ```string.distance.hamming[|.]``` returning the distance between two strings using Hamming algorithm
* Function ```string.distance.sift3[|.]``` returning the distance between two strings using Damerau-Sift3 algorithm
* Function ```string.fuzzymatch[|.]``` returning the fuzzy matching between two strings.
* Function ```system.setproctitle``` Setting process title for supported OS.
