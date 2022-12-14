Setup
  $ source "$TESTDIR"/setup.sh

Set up a kdconfig which allows us to inspect the current directory
  $ cat <<EOF > .kdconfig
  > {
  >   "cat-argsfile": {
  >     "cmd": "cat {{ARGSFILE}}",
  >     "help": "Prints out the autogenerated argsfile"
  >   },
  >   "print": {
  >     "cmd": "echo {{ARGS}}",
  >     "help": "Prints the CLI arguments"
  >   },
  >   "print-with-prelude": {
  >     "cmd": "echo Provided arguments: {{ARGS}}",
  >     "help": "Prints the CLI arguments with a prelude"
  >   }
  > }
  > EOF

Run 'kd print' which uses the {{ARGS}} magic arg
  $ kd print Hello World!
  Hello World!

Run 'kd cat-argsfile' which uses the {{ARGSFILE}} magic arg
  $ kd cat-argsfile Hello World!
  Hello
  World!

Run 'kd print-with-prelude' which mixes {{ARGS}} with static arguments
  $ kd print-with-prelude This is a --test
  Provided arguments: This is a --test
