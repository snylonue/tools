'use_strict'

var random=false;

function random_play() {
	var pl,pos;
	if (random) {
		pl=mp.get_property_number('playlist-count');
		do {
			mp.command('playlist-shuffle');
			pos=mp.get_property_number('playlist-pos-1');
		} while(pl===pos);
	}
}
function random_play_control() {
	random=!random;
	mp.osd_message('Random: '+(random?'yes':'no'));
}

mp.register_event('start-file',random_play)
mp.add_key_binding('y','random_control',random_play_control)