#!/usr/bin/env sh

inputFile=input
idx=0
declare -a inputs
declare -a values

set -e

if [[ "$1" = "-v" ]]
then
  shift 1
  exec 3>&1
  exec 4>/dev/null
elif [[ "$1" = "-vv" ]]
then
  shift 1
  exec 3>&1
  exec 4>&1
elif [[ "$1" = "-vvv" ]]
then
  shift 1
  set -x
  exec 3>&1
  exec 4>&1
else
  exec 3>/dev/null
  exec 4>/dev/null
fi

trap cleanup 0

function cleanup {
  exec 3>&-
  exec 4>&-
}

function readInput {
  while read line
  do
    inputs[$idx]=$line
    idx=$((idx+1))
  done < $inputFile
  
  idx=$((idx-1))
}

function tryTotal {
  local levels=$1
  local total=$2
  local index=$3
  local endIndex=$4
  local val
  local nestVal

  echo "New loop: levels=$levels total=$total index=$index endIndex=$endIndex" >&3

  if [ $total -lt 1 ]
  then
    return -1
  fi

  while [ $index -le $endIndex ]
  do
    val=${inputs[$index]}
    if [ $levels -eq 1 ]
    then
      if [ $val -eq $total ]
      then
        echo "values[$levels]=$val" >&3
        values[$levels]=$val
        return 0
      fi
    else
      if [ $total -le $val ]
      then
        echo "Circuit break on level $levels as ($index)$val >= $total" >&4
      elif tryTotal $((levels-1)) $((total-val)) $((index+1)) $((endIndex+1))
      then
        echo "values[$levels]=$val" >&3
        values[$levels]=$val
        return 0
      fi
    fi
    echo "No match at level $levels for index $index and total $total" >&4
    index=$((index+1))
  done
  echo "No match at level $levels for total $total" >&4
  return -1
}

levels=$1
total=$2

readInput

if tryTotal $levels $total 0 $((idx-levels+1))
then
  echo Values found for $1 levels summing to $2:
  echo ${values[@]}
  product=1
  index=${#values[@]}
  while [ $index -ge 1 ]
  do
    value=${values[$index]}
    product=$((product*value))
    index=$((index-1))
  done
  echo Product is $product
else
  echo No values found for $1 levels summing to $2
fi
