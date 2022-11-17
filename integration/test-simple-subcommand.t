Setup
  $ source "$TESTDIR"/setup.sh

Set up a kdconfig using a fixture
  $ cp "$TESTDIR"/fixtures/basic-three-entry-kdconfig.json .kdconfig

Run subcommands
  $ kd example1
  Example 1!

  $ kd example2
  Example 2!

  $ kd third-example
  Third Example!

Run with an invalid subcommand
  $ kd does-not-exist
  No subcommand found called does-not-exist.
  [1]
