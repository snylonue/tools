'use_strict';

var random = {
	is_enabled: false,
	random: function() {
		if (this.is_enabled) {
			do {
				shuffle();
			} while (is_end_of_playlist());
			mp.msg.info("Playlist shuffled")
		}
	},
	set_is_enabled: function(val) {
		this.is_enabled = val;
		mp.osd_message('Random: ' + (this.is_enabled ? 'yes' : 'no'));
	},
}
var pause_current_file = {
	is_enabled: false,
	pause_current_file: function(event) {
		if (this.is_enabled && event.reason === 'eof') {
			mp.command('cycle pause');
		}
	},
	set_is_enabled: function(val) {
		this.is_enabled = val;
		mp.osd_message('Pause current file: ' + (this.is_enabled ? 'yes' : 'no'));
	},
}

function shuffle() {
	mp.command('playlist-shuffle');
}
function is_end_of_playlist() {
	return mp.get_property_number('playlist-pos-1') === mp.get_property_number('playlist-count');
}
function toggle(obj) {
	return function() {
		obj.set_is_enabled(!obj.is_enabled);
	};
}

mp.register_event('start-file', function() { random.random() });
mp.register_event('end-file', function() { pause_current_file.pause_current_file() });
mp.add_key_binding('y', 'random_control', toggle(random));
mp.add_key_binding('p', 'pause_on_finish_control', toggle(pause_current_file));