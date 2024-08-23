// Define a simple Möbius strip module

// Function to calculate a point on the Möbius strip
let pointOnMobius = (u: float, v: float) => {
  let x = (1.0 +. v /. 2.0 *. Js.Math.cos(u /. 2.0)) *. Js.Math.cos(u)
  let y = (1.0 +. v /. 2.0 *. Js.Math.cos(u /. 2.0)) *. Js.Math.sin(u)
  let z = v /. 2.0 *. Js.Math.sin(u /. 2.0)
  (x, y, z)
}
