Setup
  $ source "$TESTDIR"/setup.sh

Set up a kdconfig using a fixture
  $ cp "$TESTDIR"/fixtures/basic-three-entry-kdconfig.json .kdconfig

Run help
  $ kd --help
  kd - A per-project command runner.
  
  USAGE:
    $ kd <subcommand> <subcommand arguments>
  
  AVAILABLE SUBCOMMANDS:
  
    example1
      Just an example
  
    example2
      Another example
  
    third-example
      Last example
