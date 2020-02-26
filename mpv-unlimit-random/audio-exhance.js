'use_strict';

var random = false;
var pasue = false;

function shuffle() {
	mp.command('playlist-shuffle');
}
function is_end() {
	return mp.get_property_number('playlist-pos-1') === mp.get_property_number('playlist-count');
}
function random_play() {
	if (random) {
		do {
			shuffle();
		} while (is_end());
	}
}
function pasue_current_file(event) {
	if (pasue) {
		mp.command('cycle pause');
	}
}
function random_play_control() {
	random = !random;
	mp.osd_message('Random: ' + (random ? 'yes' : 'no'));
}
function pasue_current_file_control() {
	pasue = !pasue;
	mp.osd_message(pasue ? 'Pause current file' : 'Unpause current file');
}

mp.register_event('start-file', random_play);
mp.register_event('end-file', pasue_current_file);
mp.add_key_binding('r', 'random_control', random_play_control);
mp.add_key_binding('p', 'pause_on_finish_control', pasue_current_file_control);