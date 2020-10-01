'use_strict';

var osd = mp.osd_message;

var random = {
	is_enabled: false,
	loop_playlist: '',
	random: function() {
		if (this.is_enabled) {
			shuffle();
		}
	},
	set_is_enabled: function(val) {
		this.is_enabled = val;
		if (this.is_enabled) {
			mp.set_property('loop-playlist', 'yes');
		} else {
			mp.set_property('loop-playlist', this.loop_playlist);
		}
		osd('Random: ' + (this.is_enabled ? 'yes' : 'no'));
	},
}
var pause_current_file = {
	is_enabled: false,
	pause_current_file: function(event) {
		if (this.is_enabled && event.reason === 'eof') {
			mp.command('cycle pause');
			this.is_enabled = false;
		}
	},
	set_is_enabled: function(val) {
		this.is_enabled = val;
		osd('Pause current file: ' + (this.is_enabled ? 'yes' : 'no'));
	},
}

function shuffle() {
	mp.command('playlist-shuffle');
	mp.msg.info("Playlist shuffled");
}
function is_end_of_playlist() {
	return mp.get_property_number('playlist-pos-1') === mp.get_property_number('playlist-count');
}
function toggle(obj) {
	return function() {
		obj.set_is_enabled(!obj.is_enabled);
	};
}

random.loop_playlist = mp.get_property('loop-playlist');
mp.register_event('start-file', function() { random.random() });
mp.register_event('end-file', function(event) { pause_current_file.pause_current_file(event) });
mp.add_key_binding('y', 'random_control', toggle(random));
mp.add_key_binding('p', 'pause_on_finish_control', toggle(pause_current_file));