#!/usr/bin/env node

const fs = require('fs')
const path = require('path')

const strict = process.env.STRICT == 1 || false
const debug = process.env.DEBUG == 1 || false

const hair = /^#[0-9a-f]{6}$/
const eyes = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
const pidFormat = /^[0-9]{9}$/
const validators = {
  byr(v) {
    const y = parseInt(v)
    const res = y >= 1920 && y <= 2002
    debug && console.log(`byr ${v} ${res}`)
    return res
  },
  iyr(v) {
    const y = parseInt(v)
    const res = y >= 2010 && y <= 2020
    debug && console.log(`iyr ${v} ${res}`)
    return res
  },
  eyr(v) {
    const y = parseInt(v)
    const res = y >= 2020 && y <= 2030
    debug && console.log(`eyr ${v} ${res}`)
    return res
  },
  hgt(v) {
    const u = v.slice(-2)
    const h = parseInt(v.slice(0, -2))
    let res = true
    switch (u) {
      case "cm":
        if (h < 150 || h > 193) {
          res = false
        }
        break
      case "in":
        if (h < 59 || h > 76) {
          res = false
        }
        break
      default:
        res = false
    }
    debug && console.log(`hgt ${v} ${res}`)
    return res
  },
  hcl(v) {
    const res = hair.test(v)
    debug && console.log(`hcl ${v} ${res}`)
    return res
  },
  ecl(v) {
    const res = eyes.includes(v)
    debug && console.log(`ecl ${v} ${res}`)
    return res
  },
  pid(v) {
    const res = pidFormat.test(v)
    debug && console.log(`pid ${v} ${res}`)
    return res
  },
}
const fields = Object.keys(validators)

fs.readFile(path.join(__dirname, '/input'), 'utf8', (err, data) => {
    const lines = data.split('\n')
    let passport = {}
    let validPassports = 0

    for (const line of lines) {
      if (line.length == 0) {
        const valid = fields
          .map(f => {
            const fValue = passport[f]
            if (fValue && fValue.length > 0) {
              return !strict || validators[f](fValue)
            } else {
              debug && console.log(`Missing field for ${f}`)
            }

            return false
          })
          .reduce((acc, next) => {
            return acc && next
          }, true)

          debug && console.log(`Passport was ${valid ? '' : 'in'}valid: ${JSON.stringify(passport)}`)

        if (valid) {
          validPassports++
        }
        passport = {}
      } else {
        line.split(" ")
          .forEach(field => {
            [ f, v ] = field.split(":")
            passport[f] = v
          })
      }
    }

    console.log(`Valid passports: ${validPassports}`)
})