#!/bin/bash

PYTHON=$(python -c "import sys; print(sys.executable)")
echo "Building angr-management with Python at $PYTHON"

rm -rf dist
mkdir dist

# Build launcher
pushd launcher
PYO3_PYTHON=$PYTHON cargo build --release
popd
cp launcher/target/release/launcher dist/angr-management

# Install site-packages
mkdir dist/site-packages
$PYTHON -m pip install -t dist/site-packages angr-management
# TODO: This is a hack because z3 doesn't declare setuptools as a dependency
$PYTHON -m pip install -t dist/site-packages setuptools

# Copy libpython
cp $(dirname $PYTHON)/../lib/libpython* dist/

# Copy stdlib
cp -r $(dirname $PYTHON)/../lib/python*/ dist/stdlib
