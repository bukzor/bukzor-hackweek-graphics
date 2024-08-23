open Mobius

// Create a simple visualization of the Möbius strip
let drawMobius = () => {
  let points = Belt.Array.make(0, (0.0, 0.0, 0.0))

  for u in 0 to 360 {
    let uRadians = Js.Math._PI *. (float_of_int(u) /. 180.0)
    [-1, 1]->Js.Array2.forEach(v => {
      points->Belt.Array.push(pointOnMobius(uRadians, float_of_int(v)))
    })
  }

  // Displaying points on console (for now)
  points->Belt.Array.forEach(point => {
    let (x, y, z) = point
    Js.log({"x": x, "y": y, "z": z})
  })
}

drawMobius()
