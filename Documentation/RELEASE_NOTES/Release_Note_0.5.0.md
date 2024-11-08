Here is what's new:
* ```{URL} use``` load BUND script from URL resource provided in stack and execute it.
* ```use.``` load BUND script from URL resource provided in the workbench and execute it.
* Function ```string.wildcard``` and ```string.wildcard.``` returned TRUE or FALSE if string on stack or workbench matches or not matches  pattern on stack.
* Function ```string.regex``` and ```string.regex.``` returned TRUE or FALSE if string on stack or workbench matches or not matches  regex pattern on stack.
* Function ```string.regex.matches``` and ```string.regex.matches.``` returned list of matches to regex pattern on stack against data on stack or workbench.
* Function ```string.regex.split``` and ```string.regex.split.``` returned list of values resulting to regex pattern split against data on stack or workbench.
* Function ```fs.cp``` copying files specified as string or list to target
* Function ```fs.mv``` moving files specified as string or list to target
* Function ```fs.rm``` removing files specified as string or list
* Function ```file.write``` writing content of the string to the file
* New CLI command ```load``` When you pass a mandatory CLI option ```--world``` it will load this WORLD file and execute all bootstrap scripts in alphabetical order.
* Function ```string.prefix[.]``` detecting prefix of the string
* Function ```string.suffix[.]``` detecting suffix of the string
