
function x() {
  let a = 1
  let count = () => {
    a += 1
  }

  count()

  return a
}

let result = x()

if (result == 2)
  console.log("Success: result = " + result)
else
  console.log("Failure: result = " + result)
