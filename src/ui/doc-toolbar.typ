#import "theme.typ": theme
#import "toolbar.typ": button, toolbar

#let current_page = "1 / 16"

#set page(
  width: 10cm,
  height: 2cm,
  margin: theme.gap / 2,
)

#toolbar(
  button(active: true)[<<<], // this is a mock of what can be set by rust
  button[#current_page],
  button[>>>],
)

