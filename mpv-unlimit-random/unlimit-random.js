'use_strict'

var random=false;
var yn={true:"yes",false:"no"};

function random_play() {
	if (random) {
		var plct=mp.get_property("playlist-count");
		var pos=mp.get_property("playlist-pos-1");
		mp.command("playlist-shuffle");
		while (plct==pos) {
			mp.command("playlist-shuffle");
			pos=mp.get_property("playlist-pos-1");
		}
	}
}
function random_play_control() {
	random=!random;
	mp.osd_message("Random: "+yn[random.toString()]);
}

mp.register_event("start-file",random_play)
mp.add_key_binding('y',"random_control",random_play_control)