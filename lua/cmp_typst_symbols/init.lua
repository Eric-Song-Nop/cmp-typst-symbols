local source = {}

local emoji_items = require("cmp_typst_symbols.items_emoji")
local typst_items = require("cmp_typst_symbols.items_typst")
-- merge items in typst and emoji
local mixed_items = vim.tbl_deep_extend("keep", typst_items, emoji_items)

source.new = function()
	return setmetatable({}, { __index = source })
end

source.get_trigger_characters = function()
	return { "\\" }
end

source.get_keyword_pattern = function()
	return "\\\\[^[:blank:]]*"
end

source.complete = function(self, request, callback)
	local option = request.option
	vim.print(option)
	if not self.items then
		if option.use_emoji then
			self.items = mixed_items
		else
			self.items = typst_items
		end
	end
	vim.print(self.items)
	callback(self.items)
end

return source
