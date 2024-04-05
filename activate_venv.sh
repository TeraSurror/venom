#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <virtual_environment_name>"
    exit 1
fi

VENV_NAME = "$1"

if [ -d "$VENV_NAME" ]; then
    source "$VENV_NAME/bin/activate"
    echo "Virtual environment activated."
else
    echo "Virtual environment does not exist."
    exit 1
fi
