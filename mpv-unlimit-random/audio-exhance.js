'use_strict';

var audio_exhance = {
	random : false,
	pasue : false,
};

function pl_ending() {
	return mp.get_property_number('playlist-pos-1') === mp.get_property_number('playlist-count');
}
function random_play() {
	var pl,pos;
	if (audio_exhance.random) {
		do {
			mp.command('playlist-shuffle');
		} while(pl_ending());
	}
}
function pasue_on_finish() {
	if (audio_exhance.pasue) {
		mp.command('cycle pause');
	}
}
function random_play_control() {
	audio_exhance.random = !audio_exhance.random;
	mp.osd_message('Random: ' + (audio_exhance.random ? 'yes' : 'no'));
}
function pause_on_finish_control() {
	audio_exhance.pasue = !audio_exhance.pasue;
	mp.osd_message('Pause on finish: ' + (audio_exhance.pasue ? 'yes' : 'no'));
}

mp.register_event('start-file', random_play);
mp.register_event('end-file', pasue_on_finish);
mp.add_key_binding('y', 'random_control', random_play_control);
mp.add_key_binding('p', 'pause_on_finish_control', pause_on_finish_control);