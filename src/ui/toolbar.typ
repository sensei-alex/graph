#import "theme.typ": theme

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

#let toolbar(..content) = context {
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

