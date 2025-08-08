// Theme (move into a separate file)
#let theme = (
  gap: 2pt,
  icon_stroke: 2pt,
  crust: black,
  base: white,
  text: black,
  active: (
    base: black,
    text: white,
  ),
  icon_size: 1em,
)

#let button(active: false, content) = rect(
  width: 100%,
  height: 100%,
  stroke: theme.crust + theme.gap,
  fill: if active { theme.active.base } else { theme.base },
  align(
    horizon + center,
    text(
      size: 16pt,
      weight: "bold",
      font: "jetbrains mono",
      fill: if active { theme.active.text } else { theme.text },
      content,
    ),
  ),
)

#let button_circle(active: false) = button(
  active: active,
  circle(
    radius: theme.icon_size / 2,
    stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
  ),
)

#let button_rect(active: false) = button(
  active: active,
  square(
    size: theme.icon_size, 
    stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
  ),
)

#let button_polygon(active: false) = button(
  active: active,
  polygon.regular(
    size: theme.icon_size, 
    vertices: 6,
    stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
  ),
)

#let button_image(active: false) = button(
  active: active,
  square(
    size: theme.icon_size,
    inset: 0pt,
    stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
  )[
    #place(
      bottom + right,
      polygon(
        stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
        (25%, 100%),
        (100%, 100%),
        (100%, 75%),
        (75%, 50%),
      ),
    )
    #place(
      top + left,
      dx: 15%,
      dy: 15%,
      circle(
        stroke: theme.icon_stroke + if active { theme.active.text } else { theme.text },
        radius: theme.icon_size / 8,
      ),
    )
  ],
)

#let ui_row(..content) = context {
  let is_portrait = page.width / page.height < 1
  let width = if is_portrait { page.height } else { page.width }
  let height = if is_portrait { page.width } else { page.height }
  let angle = if is_portrait { 270deg} else { 0deg }

  align(
    center + horizon,
    rotate(
      angle,
      box(
        width: width,
        height: height,
        grid(
          columns: content.pos().len(),
          stroke: theme.crust + theme.gap,
          ..content,
        ),
      )
    )
  )
}

#let menu_line(active: false, content) = rect(
  stroke: theme.crust + theme.gap,
  fill: if active { theme.active.base } else { theme.base },
  inset: 1em,
  width: 100%,
  align(
    horizon,
    text(
      size: 16pt,
      weight: "bold",
      font: "jetbrains mono",
      fill: if active { theme.active.text } else { theme.text },
      content,
    ),
  ),
)

#let menu(..args) = rect(
  width: 100%,
  fill: theme.base,
  stroke: theme.crust + theme.gap,
  inset: 0pt,
  grid( ..args ),
)

// Variables set by rust
#let page_format = "16x9"

#set page(
  width: 10cm,
  height: 2cm,
  margin: theme.gap / 2,
)

// ------------ >8 ------------

#ui_row(
  button(active: true)[<<<], // this is a mock of what can be set by rust
  button[#page_format],
  button[>>>],
)

#pagebreak() // actually, again, a separate file

// the top toolbar

#ui_row(
  [#button_circle(active: true) <action_circle>],
  [#button_rect() <action_rect>],
  [#button_polygon() <action_polygon>],
  [#button_image() <action_image>],
)

#pagebreak()

#set page(
  width: 10cm,
  height: 15cm,
)

#menu(
  menu_line(active: true)[search notes],
  menu_line[code],
  menu_line[import],
  menu_line[export],
  menu_line[shortcuts],
  menu_line[language],
)
