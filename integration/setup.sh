#!/bin/bash
# Setup script called at the start of every Cram test

shopt -s expand_aliases

if [ -z "$KD_PATH" ]; then
  echo "Missing kd binary. This shouldn't happen if you are using `integration/run-cram.sh`"
  exit 1
fi

# Make an alias for our app to make tests look nice
alias kd=\'"$KD_PATH"\'

# By default Cram gives the same temp directory for every test, which is
# annoying if you run multiple tests at once. Add the $TESTFILE to the path
# and reset all the variables
TEST_RUNNER_TMP="$CRAMTMP"

mkdir -p "$TEST_RUNNER_TMP/tmp-for-$TESTFILE"
export CRAMTMP="$TEST_RUNNER_TMP/tmp-for-$TESTFILE"
export TMP="$CRAMTMP"
export TEMP="$CRAMTMP"
export TEMPDIR="$CRAMTMP"