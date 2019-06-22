#!/bin/sh

# Run this script to generate the correct testcases
# Note that, the correct testcases are already tracked, so
# running this script should have no noticeable change, assuming
# that you are up-to-date with master.

bedtools unionbedg -i 1.bg 2.bg 3.bg > 1+2+3.bg
bedtools unionbedg -i empty-1.bg empty-2.bg > empty-1+2.bg
#TODO: replace this when you have a "random"

bedtools unionbedg -i 1.bg 2.bg 3.bg > 1+2+3.bg
bedtools unionbedg -i empty-1.bg empty-2.bg > empty-1+2.bg