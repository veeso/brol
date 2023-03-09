#!/bin/bash

EMAIL=$1
CNT=$2

if [ -z "$CNT" ] || [ -z "$EMAIL" ]; then
	echo "Usage: $0 <email> <subscriptions>"
	exit 1
fi

i=0

while [ $i -lt $CNT ]; do
	python3 ./spaziogrigio.py $EMAIL
	let i=i+1
done

