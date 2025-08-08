// rust
#let display_diagonal = 27
#let w_px = 1920
#let h_px = 1080

// ------------ >8 ------------

#let ppi = calc.sqrt(calc.pow(w_px, 2) + calc.pow(h_px, 2)) / display_diagonal

#set page(
  width: 1in * (w_px / ppi),
  height: 1in / ppi * h_px,
  fill: black,
)

#set text(size: 12pt, fill: white, font: "jetbrains mono")

Hello, world

PPI: #ppi
width: #{1in * (w_px / ppi)}

#rect(width: 15cm, height: 5cm, fill: green)
