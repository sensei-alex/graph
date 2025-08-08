#import "theme.typ": theme
#import "toolbar.typ": button, toolbar

#let page_format = "16x9"

#set page(
  width: 10cm,
  height: 2cm,
  margin: theme.gap / 2,
)

#toolbar(
  button(active: true)[<<<], // this is a mock of what can be set by rust
  button[#page_format],
  button[>>>],
)

