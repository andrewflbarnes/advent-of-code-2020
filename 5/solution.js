#!/usr/bin/env node

const fs = require('fs')
const path = require('path')

const debug = process.env.DEBUG == 1 || false

fs.readFile(path.join(__dirname, '/input'), 'utf8', (err, data) => {
    const lines = data.split('\n')
    let max = 0
    let seated = []
    let possibleSeats = []

    for (const line of lines) {
      const binRow = line.slice(0, 7)
      const binCol = line.slice(7)

      const row = parseBin(binRow)
      const col = parseBin(binCol)
      const seatId = row * 8 + col
      debug && console.log(`${seatId}: ${row}, ${col}`)

      seated.push(seatId)

      max = seatId > max ? seatId : max
    }

    console.log(`Highest seat ID: ${max}`)

    let preSeat
    let maybeMySeat = seated.includes(8)
    let postSeat = seated.includes(9)

    const uLimit = 127 * 8
    for (i = 10; i < uLimit; i++) {
      preSeat = maybeMySeat
      maybeMySeat = postSeat
      postSeat = seated.includes(i)

      if (preSeat && postSeat && !maybeMySeat) {
        possibleSeats.push(i - 1)
      }
    }

    console.log(`Possible seats: ${possibleSeats}`)
})

function parseBin(str) {
  let res = 0
  const strLen = str.length
  for (i = 0; i < strLen; i++) {
    const char = str.charAt(strLen - i - 1)
    if (char === 'R' || char === 'B') {
      res += 1 << i
    }
  }

  return res
}