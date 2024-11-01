Here is what's new:
* ```--stack``` global CLI option will set a current stack stack name before computation
* ```--bootstrap``` global CLI option will set a list of files accessible locally or via http/https to bootstrap BUND with initial scripts. In the following exampe, BUND will run script located in /tmp/examples before launching main shell/script execution.
```shell
bund --bootstrap file:///tmp/examples/helloworld_lambda.bund shell
```
