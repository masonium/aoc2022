#!/bin/bash

day=$(TZ=":America/New_York" date +"%d")
day_single=$(echo -n "$day" | sed "s/^0//g")

session=$(head -n 1 session.txt)

curl https://adventofcode.com/2022/day/$day_single/input --cookie session=$session > /home/mason/workspace/aoc2022/input/$day.in
