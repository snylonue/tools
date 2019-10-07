local msg = require "mp.msg"
local utils = require "mp.utils"
local options = require "mp.options"

local random = false

function random_play()
	if random then
		mp.command("playlist-shuffle")
	end
end
function random_play_control()
	random = not random
	mp.osd_message("Random: " .. (random and "yes" or "no"))
end
mp.register_event("start-file", random_play)
mp.add_key_binding('y', "random_control", random_play_control)