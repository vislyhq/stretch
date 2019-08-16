
# Stretched
Python bindings for Stretch - A high performance flexbox implementation written in rust.



## Overview

These bindings were translated from the swift bindings.

- See the tests in ./tests/test.py for examples on how to use the bindings.



Bindings are generated using pyo3 and pyo3-pack.

- Please see the relevant documentation for more information on how to build the bindings.



## Basic Build Instructions

Python 3.5 to 3.7 should be supported.

- You will need pyenv and pyenv virtualenv setup, activate your virtual environment.
- install the requirements `pip install -r ./requirements_dev.txt`
- compile and install the development version directly into your environment with `pyo3-pack develop`
- compile a release version with `pyo3-pack build`



