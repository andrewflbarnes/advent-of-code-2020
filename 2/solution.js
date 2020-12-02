const fs = require('fs')
const path = require('path')

fs.readFile(path.join(__dirname, '/input'), 'utf8', (err, data) => {
    console.log(data.split('\n')[0])
    const lines = data.split('\n')
    const valid = lines.filter((line, i) => {
        if (line === null || line.length === 0) {
            return
        }
        const values = line.split(' ')
        const count = values[0].split('-')
        const min = count[0]
        const max = count[1]
        const letter = values[1].slice(0, values[1].length - 1)
        const pass = values[2]

        // console.log(`min=${min} max=${max} letter=${letter} password=${pass}`)
        
        const occurrences = (pass.match(new RegExp(letter, 'g')) || []).length
        console.log(`pass=${pass} letter=${letter} occurrences=${occurrences} min=${min} max=${max}`)

        return occurrences >= min && occurrences <= max
    })
    .length

    console.log(valid)
})
