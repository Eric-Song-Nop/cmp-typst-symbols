# cmp-typst-symbols

Cmp source for symbols input with Typst names.

## Installation

```lua
-- Lazy.nvim
{
    "hrsh7th/nvim-cmp",
    opts = function(_, opts)
        table.insert(opts.sources, { name = "typst_symbols", option = { use_emoji = false } })
    end,
    dependencies = {
        { "Eric-Song-Nop/cmp-typst-symbols" },
    },
}
```
