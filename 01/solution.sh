#!/usr/bin/env sh

inputFile=input
idx=0
declare -a inputs

while read line
do
  inputs[$idx]=$line
  idx=$((idx+1))
done < $inputFile

idx=$((idx-1))

i=0
iLimit=$((idx-1))
j=0
jLimit=$((idx))

while [ $i -lt $iLimit ]
do
  while [ $j -lt $jLimit ]
  do
    iVal=${inputs[$i]}
    jVal=${inputs[$j]}

    if [[ $((iVal+jVal)) = 2020 ]]
    then
      echo "($i)$iVal ($j)$jVal"
      echo $((iVal*jVal))
      break 2
    fi

    j=$((j+1))
  done
  i=$((i+1))
  j=$((i+1))
done
