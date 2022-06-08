#!/bin/bash

for line in $(cat webapp/.env)
do
  export $line
  code .
done
