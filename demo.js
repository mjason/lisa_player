const player = require(".")

player.play("./1.wav").then((data) => {
  console.log(data)
})

console.log("manjia")