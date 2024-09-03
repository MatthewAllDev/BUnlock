#!/bin/bash

./uninstall.sh && cd .. && cargo build --release && cd tools && ./install.sh