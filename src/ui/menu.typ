#import "theme.typ": theme

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
    )[
      #if active [> ]
      #content
    ],
  ),
)

#let menu(..args) = rect(
  width: 100%,
  fill: theme.base,
  stroke: theme.crust + theme.gap,
  inset: 0pt,
  grid( ..args ),
)

// the menu component is going te be rendered by the rust part
// so it must be called menu
// then the page background will be removed
// so this is a mock:
#set page(
  width: 10cm,
  height: 10cm,
  margin: theme.gap,
)

// I want the menu nav to start from the center
#menu(
  menu_line[import],
  menu_line[export],
  menu_line[code],
  menu_line(active: true)[search notes],
  menu_line[help],
  menu_line[shortcuts],
  menu_line[language],
)

