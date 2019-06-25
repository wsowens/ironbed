#!/bin/sh

commit=$(git rev-parse HEAD)
name="$1"
command="$2"
filename="bench/${name}.${commit}.time"
tmpname=$(mktemp)
eval "/usr/bin/time -po ${tmpname} ${command}"
echo "$command" | cat - "${tmpname}" >> ${filename}
rm "$tmpname"
