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
        const positions = values[0].split('-')
        const letter = values[1].slice(0, values[1].length - 1)
        const pass = values[2]

        console.log(`Line ${i}: letter=${letter} positions=${positions}`)
        
        const count = positions.filter(position => {
            return pass.charAt(position - 1) === letter ? 1 : 0
            
        }).length

        return count === 1
    })
    .length

    console.log(valid)
})
