#!/bin/sh
command time --format 'real %es\nuser %Us\nsys  %Ss\nrss  %Mk' "$@"
