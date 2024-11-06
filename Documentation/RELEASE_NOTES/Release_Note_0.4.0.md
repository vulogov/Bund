Here is what's new:
* ```--stack``` global CLI option will set a current stack stack name before computation
* ```--bootstrap``` global CLI option will set a list of files accessible locally or via http/https to bootstrap BUND with initial scripts. In the following example, BUND will run script located in /tmp/examples before launching main shell/script execution.
```shell
bund --bootstrap file:///tmp/examples/helloworld_lambda.bund shell
```
* ```{filename} save``` saving state of the BUND VM into external "WORLD" file.
* ```{filename} load``` loading state of the BUND VM from external "WORLD" file.
* ```{filename} bootstrap``` loading state of the BUND VM from external "WORLD" file with execution of the BOOTSTRAP scripts stored in the "WORLD" file.
* ```{filename} save.[aliases|stack|lambdas]``` saving elements of the BUND VM into external "WORLD" file.
* ```{filename} load.[aliases|stack|lambdas]``` load elements of the BUND VM from external "WORLD" file.
* ```{filename} save.script``` save script to the BOOTSTRAP region of the "WORLD" file.
* ```{filename} load.script``` load script from the BOOTSTRAP region of the "WORLD" file to the stack.
