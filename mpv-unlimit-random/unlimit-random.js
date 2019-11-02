'use_strict'

var random=false;

function zrandom(nmax) {
	return Math.round(Math.random()*nmax);
}
function random_play() {
	var pl,pos,setpos;
	if (random) {
		pl=mp.get_property_number('playlist-count');
		pos=mp.get_property_number('playlist-pos');
		do {
			setpos=zrandom(pl);
		} while(setpos===pos);
		mp.set_property('playlist-pos',setpos.toString());
	}
}
function random_play_control() {
	random=!random;
	mp.osd_message('Random: '+(random?'yes':'no'));
}

mp.register_event('end-file',random_play)
mp.add_key_binding('y','random_control',random_play_control)
