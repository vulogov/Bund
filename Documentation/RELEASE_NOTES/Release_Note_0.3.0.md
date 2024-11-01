* Introduction of the "WORLD" - file to which you can save the state of BUND virtual machine.
* save.aliases - Saving aliases to the "WORLD"
* save.lambdas - Saving user-defined named lambda functions to the "WORLD"
* save.stacks - Saving state of the stack
* save - Save entire state of the BUND VM
* load.aliases - Loading aliases from the "WORLD"
* load.lambdas - Loading named lambdas from the "WORLD"
* load.stacks - Loading stack data into VM
* load - Loading entire VM state from the "WORLD" file
* CLI option bund shell --as-script. If specified, all inputs will be treated as script and executed throuh VM::eval() rather than VM::run() function. The most prominent feature is the top value of the stack or workbench will be intact if --as-script specified.
* args - Pushing to stack arguments passed for shell and script subcommand after ```--```
 
