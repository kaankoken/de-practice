#! /bin/bash

echo "Checking pip..."
default_pip="pip"
# Check whether pip command exist
if [ -x "$(command -v python3 -m pip --version)" ]
then
    default_pip="pip3"
fi

echo "Checking requirements.txt..."
$default_pip install -U -r requirements.txt

echo "Recreating requirements.txt..."
$default_pip freeze > requirements.txt

echo "Done."
