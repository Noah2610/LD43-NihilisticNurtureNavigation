const default_settings = {
  screen: {
    w: parseInt($('#grid_wrapper').css("width")),
    h: parseInt($('#grid_wrapper').css("height"))
  },
  box_size: {
    w: 32, h: 32
  },
  block_size: {
    w: 32, h: 32
  },
  block_offset: {
    x: 0, y: 0
  },
  room_size: {
    w: 1280, h: 720
  },
  colors: {}
};

var settings = undefined;

$(document).ready(function () {
  settings = deep_copy(default_settings);
});
