var cur_keys = [];
var mult = 0;
var controls = {};
const valid_keys = [
  "h","j","k","l",
  "H","J","K","L",
  "ArrowLeft","ArrowDown","ArrowUp","ArrowRight",
  "g","G",
  "M","m",
  "0","$",
  " ","Enter",
  "x","Backspace",
  "p","b",
  "d","D",
  "s","S",
  "a","A",
  "q","Q",
  "n","N",
  "r","R",
  "O","W"
];
const keys_max_length = 2;
const box_size_step = 8;
const min_box_size = 8;

function update_key_display() {
  var m = "";
  if (mult > 0) {
    m = String(mult);
  }
  $('#key_display').text(m + cur_keys.join(""));
}

function clear_keys() {
  cur_keys = [];
  mult = 0;
  update_key_display();
}

function add_key(key) {
  if (cur_keys.length >= keys_max_length)
    clear_keys();
  cur_keys.push(key);
  update_key_display();
}

function switch_block_size() {
  var w = settings.block_size.w;
  var h = settings.block_size.h;
  settings.block_size.w = h;
  settings.block_size.h = w;
  update_panel();
}

function switch_block_offset() {
  const x = settings.block_offset.x;
  const y = settings.block_offset.y;
  settings.block_offset.x = y;
  settings.block_offset.y = x;
  update_panel();
}

function increase_box_size(axis) {
  var step = box_size_step;
  if (axis == "W" || axis == "H")
    axis = axis.toLowerCase();
  settings.box_size[axis] += step;
  update_panel();
  update_grid();

}
function decrease_box_size(axis) {
  var step = box_size_step;
  if (axis == "W" || axis == "H")
    axis = axis.toLowerCase();
  if (settings.box_size[axis] - step >= min_box_size) {
    settings.box_size[axis] -= step;
    update_panel();
    update_grid();
  }
}

function increase_block_size(axis) {
  var step = Math.round(settings.box_size[axis.toLowerCase()] / 4);
  if (axis == "W" || axis == "H") {
    axis = axis.toLowerCase();
    step = settings.box_size[axis];
  }
  settings.block_size[axis] += step;
  update_panel();
}
function decrease_block_size(axis) {
  var step = Math.round(settings.box_size[axis.toLowerCase()] / 4);
  if (axis == "W" || axis == "H") {
    axis = axis.toLowerCase();
    step = settings.box_size[axis];
  }
  if (settings.block_size[axis] - step > 0) {
    settings.block_size[axis] -= step;
    update_panel();
  }
}

function increase_block_offset(axis) {
  const side = axis.toLowerCase() == "x" ? "w" : "h";
  var step = Math.round(settings.box_size[side] / 4);
  if (axis == "X" || axis == "Y") {
    axis = axis.toLowerCase();
    step = settings.box_size[side];
  }
  settings.block_offset[axis] += step;
  update_panel();
}
function decrease_block_offset(axis) {
  const side = axis.toLowerCase() == "x" ? "w" : "h";
  var step = Math.round(settings.box_size[side] / 4);
  if (axis == "X" || axis == "Y") {
    axis = axis.toLowerCase();
    step = settings.box_size[side];
  }
  if (settings.block_offset[axis] - step >= 0) {
    settings.block_offset[axis] -= step;
    update_panel();
  }
}

function next_block() {
  const select = $('#panel__block_selector__select');
  const cur = select.val();
  var next_i = 0;
  const children = select.children('option');
  children.each(function (i) {
    if ($(this).val() == cur) {
      next_i = i + 1;
      return;
    }
  });
  if (next_i >= children.length) {
    next_i = next_i - children.length;
  }
  select.val($(children.get(next_i)).val());
  select.trigger("change");
}
function prev_block() {
  const select = $('#panel__block_selector__select');
  const cur = select.val();
  var prev_i = 0;
  const children = select.children('option');
  children.each(function (i) {
    if ($(this).val() == cur) {
      prev_i = i - 1;
      return;
    }
  });
  if (prev_i < 0) {
    prev_i = children.length + prev_i;
  }
  select.val($(children.get(prev_i)).val());
  select.trigger("change");
}

function reset(target) {
  switch (target) {
    case "box_size":
      settings.box_size = deep_copy(default_settings.box_size);
      update_grid();
      break;
    case "block_size":
      //settings.block_size = deep_copy(default_settings.block_size);
      settings.block_size = deep_copy(settings.box_size);
      break;
    case "block_offset":
      settings.block_offset = deep_copy(default_settings.block_offset);
      break;
    case "all":
      reset("box_size");
      reset("block_size");
      reset("block_offset");
      break;
  }
  update_panel();
}

function move_highlight(dir) {
  const highlight = $('#grid__block__highlight');
  const x = parseInt(highlight.data("x")) - (parseInt(highlight.data("x")) % settings.box_size.w);
  const y = parseInt(highlight.data("y")) - (parseInt(highlight.data("y")) % settings.box_size.h);
  const step = {
    x: settings.box_size.w,
    y: settings.box_size.h
  }
  switch (dir) {
    case "left":
      if ((x - step.x) >= 0)
        highlight.data("x", x - step.x);
      break;
    case "right":
      if ((x + step.x) < settings.room_size.w)
        highlight.data("x", x + step.x);
      break;
    case "up":
      if ((y - step.y) >= 0)
        highlight.data("y", y - step.y);
      break;
    case "down":
      if ((y + step.y) < settings.room_size.h)
        highlight.data("y", y + step.y);
      break;
    default:
      return;
  }
  update_highlight();
}

function remove_block_keypress() {
  const highlight = $('#grid__block__highlight');
  const x = highlight.data("x") + settings.block_offset.x;
  const y = highlight.data("y") + settings.block_offset.y;
  remove_block(x, y);
}

function move_highlight_to(target) {
  const highlight = $('#grid__block__highlight');
  var x, y, bottom, end;
  switch (target) {
    case "top":
      highlight.data("y", "0");
      break;
    case "bottom":
      bottom = (settings.room_size.h - 1) - ((settings.room_size.h - 1) % settings.box_size.h);
      highlight.data("y", bottom);
      break;
    case "start":
      highlight.data("x", "0");
      break;
    case "end":
      end = (settings.room_size.w - 1) - ((settings.room_size.w - 1) % settings.box_size.w);
      highlight.data("x", end);
      break;
    case "center":
      x = ((settings.room_size.w / 2) - 1) - (((settings.room_size.w / 2) - 1) % settings.box_size.w);
      y = ((settings.room_size.h / 2) - 1) - (((settings.room_size.h / 2) - 1) % settings.box_size.w);
      highlight.data("x", x);
      highlight.data("y", y);
      break;
    case "center_row":
      y = ((settings.room_size.h / 2) - 1) - (((settings.room_size.h / 2) - 1) % settings.box_size.w);
      highlight.data("y", y);
      break;
    default:
      return;
  }
  update_highlight();
}


function handle_keypress(event) {
  if (event.ctrlKey)
    return;

  // Return if target is inside panel
  if ($(event.target).parents('#panel').length == 1)
    return;

  // Clear keys if Escape was pressed
  if (event.key == "Escape") {
    clear_keys();
    return;
  }

  // Multiplier key - Number
  if (event.key.match(/[0-9]/) != null) {
    if (!(event.key == "0" && mult == 0)) {
      if (mult == 0)
        mult = "";
      mult = parseInt(String(mult) + event.key);
      update_key_display();
      return;
    }
  }

  // Return if not valid key
  if (!valid_keys.includes(event.key))
    return;

  add_key(event.key);

  var found_comb = false;

  var loop = 1;
  if (mult > 0)
    loop = mult;

  // Check key combination
  const comb = cur_keys.join("");
  for (var i = 0; i < loop; i++) {
    // hjkl, navigation
    Object.keys(controls.directional).forEach(function (dir) {
      if (controls.directional[dir].includes(comb)) {
        move_highlight(dir);
        found_comb = true;
        return;
      }
    });

    // Increase box size
    Object.keys(controls.increase_box_size).forEach(function (axis) {
      if (controls.increase_box_size[axis].includes(comb)) {
        increase_box_size(axis);
        found_comb = true;
        return;
      }
    });
    // Decrease box size
    Object.keys(controls.decrease_box_size).forEach(function (axis) {
      if (controls.decrease_box_size[axis].includes(comb)) {
        decrease_box_size(axis);
        found_comb = true;
        return;
      }
    });

    // Increase block size
    Object.keys(controls.increase_block_size).forEach(function (axis) {
      if (controls.increase_block_size[axis].includes(comb)) {
        increase_block_size(axis);
        found_comb = true;
        return;
      }
    });
    // Decrease block size
    Object.keys(controls.decrease_block_size).forEach(function (axis) {
      if (controls.decrease_block_size[axis].includes(comb)) {
        decrease_block_size(axis);
        found_comb = true;
        return;
      }
    });

    // Increase block offset
    Object.keys(controls.increase_block_offset).forEach(function (axis) {
      if (controls.increase_block_offset[axis].includes(comb)) {
        increase_block_offset(axis);
        found_comb = true;
        return;
      }
    });
    // Decrease block offset
    Object.keys(controls.decrease_block_offset).forEach(function (axis) {
      if (controls.decrease_block_offset[axis].includes(comb)) {
        decrease_block_offset(axis);
        found_comb = true;
        return;
      }
    });

    // Resets
    Object.keys(controls.reset).forEach(function (target) {
      if (controls.reset[target].includes(comb)) {
        reset(target);
        found_comb = true;
        return;
      }
    });

    // Move highlight to center of grid
    if (controls.center.includes(comb)) {
      move_highlight_to("center");
      found_comb = true;
    } else if (controls.center.includes(comb)) {
      // Move highlight to center of grid
      move_highlight_to("center");
      found_comb = true;
    } else if (controls.row_top.includes(comb)) {
      // Move highlight to top row
      move_highlight_to("top");
      found_comb = true;
    } else if (controls.row_bottom.includes(comb)) {
      // Move highlight to bottom row
      move_highlight_to("bottom");
      found_comb = true;
    } else if (controls.row_center.includes(comb)) {
      // Move highlight to center row
      move_highlight_to("center_row");
      found_comb = true;
    } else if (controls.row_start.includes(comb)) {
      // Move highlight to start row
      move_highlight_to("start");
      found_comb = true;
    } else if (controls.row_end.includes(comb)) {
      // Move highlight to end row
      move_highlight_to("end");
      found_comb = true;

    } else if (controls.place_block.includes(comb)) {
      // Place highlight
      place_highlight();
      found_comb = true;
    } else if (controls.remove_block.includes(comb)) {
      // Remove block
      remove_block_keypress();
      found_comb = true;

    } else if (controls.toggle_panel.includes(comb)) {
      // Toggle panel
      toggle_panel();
      found_comb = true;
    } else if (controls.toggle_borders.includes(comb)) {
      // Toggle grid borders (box-shadows)
      toggle_borders();
      found_comb = true;

    } else if (controls.switch_block_size.includes(comb)) {
      // Switch block size
      switch_block_size();
      found_comb = true;
    } else if (controls.switch_block_offset.includes(comb)) {
      // Switch block offset
      switch_block_offset();
      found_comb = true;

    } else if (controls.next_block.includes(comb)) {
      // Select next block
      next_block();
      found_comb = true;
    } else if (controls.prev_block.includes(comb)) {
      // Select previous block
      prev_block();
      found_comb = true;

    } else if (controls.load_level.includes(comb)) {
      // Load level (user needs to disable popup blocking)
      $('#load_level').click();
      found_comb = true;
    } else if (controls.save_level.includes(comb)) {
      // Save / Download level
      save_level();
      found_comb = true;
    }

    // Break out of loop if combination isn't valid
    if (!found_comb)
      break;

  }

  if (found_comb) {
    clear_keys();
    event.preventDefault();
  }

}


$(document).ready(function () {
  $.getJSON("./keybindings.json", function (json) {
    controls = json;
    document.addEventListener("keydown", handle_keypress);
  });
});
