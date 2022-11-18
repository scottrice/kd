Setup
  $ source "$TESTDIR"/setup.sh

Set up a kdconfig which allows us to inspect the current directory
  $ cat <<EOF > .kdconfig
  > {
  >   "pwd": {
  >     "cmd": "pwd",
  >     "help": "Print out the current directory"
  >   }
  > }
  > EOF

Run 'kd pwd' from the directory with the kdconfig
  $ kd pwd
  /tmp/cramtests-*/test-subdirectories.t (glob)

Now move into a subdirectory
  $ mkdir foo
  $ cd foo

And confirm 'kd pwd' still runs from the parent
  $ kd pwd
  /tmp/cramtests-*/test-subdirectories.t (glob)

Create a new kdconfig in our subdirectory
  $ cat <<EOF > .kdconfig
  > {
  >   "hello": {
  >     "cmd": "echo Hello World!",
  >     "help": "A friendly message"
  >   }
  > }
  > EOF

And confirm kd now uses the new config (by using a new command)
  $ kd hello
  Hello World!
