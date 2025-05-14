local lspconfig = require("lspconfig")

lspconfig.rust_analyzer.setup({
	settings = {
		["rust-analyzer"] = {
			cargo = {
				features = { "extism" },
			},
			checkOnSave = {
				command = "check",
				extraArgs = { "--features", "extism" },
			},
		},
	},
})
