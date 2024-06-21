#!/bin/bash

# Input hexadecimal string with '0x' prefix
hex_string="$1"

# Remove the '0x' prefix if present
hex_string=${hex_string#0x}

# Calculate the length of the string
total_length=${#hex_string}
part_length=$(expr $total_length / 3)

# Calculate start positions for second and third parts
start_second=$(expr $part_length)
start_third=$(expr $start_second + $part_length)

# Split into three parts
part1=$(echo "$hex_string" | cut -c1-$part_length)
part2=$(echo "$hex_string" | cut -c$(expr $start_second + 1)-$start_third)
part3=$(echo "$hex_string" | cut -c$(expr $start_third + 1)-$total_length)

# Print the parts
echo "aura     : 0x$part1"
echo "grandpa  : 0x$part2"
echo "imOnline : 0x$part3"
