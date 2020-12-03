#!/usr/bin/env node

const fs = require('fs')
const path = require('path')

const tree = '#'
const slopes = [
  {
    xInc: 1,
    yInc: 1, 
  },
  {
    xInc: 3,
    yInc: 1, 
  },
  {
    xInc: 5,
    yInc: 1, 
  },
  {
    xInc: 7,
    yInc: 1, 
  },
  {
    xInc: 1,
    yInc: 2, 
  },
]

fs.readFile(path.join(__dirname, '/input'), 'utf8', (err, data) => {
    const lines = data.split('\n')

    const product = slopes
      .map(slope => checkTreesHit(lines, slope))
      .reduce((total, current) => total * current, 1)

    console.log(`Product: ${product}`)
})

function display(line, { x }) {
  console.log(line)
  console.log(`${' '.repeat(x)}^`)
}

function checkTreesHit(lines, { xInc, yInc }) {
  const [ height, width ] = [ lines.length, lines[0].length ]

  console.log(`Checking slope ${height} high and ${width} wide with xInc ${xInc} and yInc ${yInc}`)

  let p = {
    x: 0,
    y: 0,
  }
  let treesHit = 0
  let line = lines[0]

  // display(line, p)

  while (p.y < height - 1) {
    p.y += yInc
    p.x += xInc
    if (p.x >= width) {
      p.x -= width
    }

    line = lines[p.y]
    if (line[p.x] === tree) {
      treesHit++
    }

    // display(line, p)
  }

  console.log(`Travelling on slope x ${xInc} y ${yInc} hits ${treesHit} trees`)
  return treesHit
}