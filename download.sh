#!/bin/bash
curl -b session=$(<.session) -o ./input/day/$1/input https://adventofcode.com/2023/day/$1/input