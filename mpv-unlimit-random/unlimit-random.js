'use_strict'

var random=false;

function random_play() {
	if (random) {
		var plct=mp.get_property('playlist-count');
		var pos;
		do {
			mp.command('playlist-shuffle');
			pos=mp.get_property('playlist-pos-1');
		} while(plct==pos);
	}
}
function random_play_control() {
	random=!random;
	mp.osd_message('Random: '+(random&&'yes'||'no'));
}

mp.register_event('start-file',random_play)
mp.add_key_binding('y','random_control',random_play_control)