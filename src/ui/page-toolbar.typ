#import "theme.typ": theme
#import "toolbar.typ": button, toolbar

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

// Mock of the thing that's going to be set by rust
#let page_format = "16x9"

#set page(
  width: 10cm,
  height: 2cm,
  margin: theme.gap / 2,
)

#toolbar(
  [#button_circle(active: true) <action_circle>],
  [#button_rect() <action_rect>],
  [#button_polygon() <action_polygon>],
  [#button_image() <action_image>],
)

