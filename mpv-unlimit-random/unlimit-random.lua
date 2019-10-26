local random = false

function random_play()
	if random then
		plct = mp.get_property("playlist-count")
		pos = mp.get_property("playlist-pos-1")
		mp.command("playlist-shuffle")
		while (pos == plct) do
			mp.command("playlist-shuffle")
			pos = mp.get_property("playlist-pos-1")
		end
	end
end
function random_play_control()
	random = not random
	mp.osd_message("Random: " .. (random and "yes" or "no"))
end

mp.register_event("start-file", random_play)
mp.add_key_binding('y', "random_control", random_play_control)