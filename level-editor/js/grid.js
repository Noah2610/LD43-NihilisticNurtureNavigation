function update_highlight(highlight = $('#grid__block__highlight')) {
  highlight.css("width", settings.block_size.w);
  highlight.css("height", settings.block_size.h);
  highlight.css("left", highlight.data("x") + settings.block_offset.x);
  highlight.css("top", highlight.data("y") + settings.block_offset.y);
}

function update_grid(grid = $('#grid')) {
  grid.empty();
  grid.css("width", settings.room_size.w + "px");
  grid.css("height", settings.room_size.h + "px");

  const box_w = settings.box_size.w;
  const box_h = settings.box_size.h;
  const style = 'width: '+ box_w +'px; height: '+ box_h +'px;';
  const box_el = '<span class="grid__box" style="'+ style +'" data-x="0" data-y="0"></span>';

  const boxes_x = Math.floor(settings.room_size.w / box_w);
  const boxes_y = Math.floor(settings.room_size.h / box_h);

  for (var row = 0; row < boxes_y; row++) {
    for (var col = 0; col < boxes_x; col++) {
      grid.append(
        box_el.replace('data-x="0"', 'data-x="'+ col * box_w +'"')
        .replace('data-y="0"', 'data-y="'+ row * box_h +'"')
      );
    }
    grid.append('<br />');
  }
}

function handle_mousemove(event, highlight) {
  const x = event.pageX;
  const y = event.pageY;
  const box_x = x - (x % settings.box_size.w);
  const box_y = y - (y % settings.box_size.h);

  //const box_el = $('#grid .grid__box[data-x="'+ box_x +'"][data-y="'+ box_y +'"]');

  if ((x >= 0 && x < settings.room_size.w) &&
    (y >= 0 && y < settings.room_size.h)) {
    highlight.data("x", box_x);
    highlight.data("y", box_y);
    update_highlight(highlight);
  }
}

function place_highlight(highlight = $('#grid__block__highlight')) {
  const box_left = highlight.css("left");
  const box_top = highlight.css("top");

  const block_wrapper = $('#blocks');

  var block_exists = false;
  block_wrapper.children('.block').each(function () {
    if ($(this).css("left") == box_left && $(this).css("top") == box_top) {
      block_exists = true;
      return;
    }
  });

  if (block_exists)
    return;

  var block = $(document.createElement('span'));
  var animation = {
    files:     $('#panel__block_animation__files').val(),
    intervals: $('#panel__block_animation__intervals').val(),
  };
  block.addClass('grid__box block');
  block.css('left',             highlight.css('left'));
  block.css('top',              highlight.css('top'));
  block.css('width',            highlight.css('width'));
  block.css('height',           highlight.css('height'));
  block.css('background-color', highlight.css('background-color'));
  block.data('instance',        current_block());
  block.data('files',           animation.files);
  block.data('intervals',       animation.intervals);

  block_wrapper.append(block);
}

function remove_block(x, y) {
  var block = false;
  const blocks = $('.block');
  blocks.each(function () {
    const b = $(this);
    const left = parseInt(b.css("left"));
    const right = parseInt(b.css("left")) + parseInt(b.css("width"));
    const up = parseInt(b.css("top"));
    const down = parseInt(b.css("top")) + parseInt(b.css("height"));
    if ( (x >= left && x < right) &&
      (y >= up && y < down) ) {
      block = b;
      return;
    }
  });

  if (block)
    block.remove();
}

function handle_click(event, highlight) {
  place_highlight(highlight);
}

function handle_rightclick(event) {
  const x = event.pageX;
  const y = event.pageY;
  remove_block(x, y);
}

function toggle_borders() {
  const boxes = $('.grid__box').not('.block').not('#grid__box__block');
  var val;
  if ($(boxes.get(0)).css("box-shadow") == "none") {
    // Add borders
    val = "inset 0px 0px 0px 2px #000";
    //val = "inline-block";
  } else {
    // Remove borders
    val = "none";
  }
  boxes.each(function () {
    const b = $(this);
    b.css("box-shadow", val);
  });
}

$(document).ready(function () {

  const grid_wrapper = $('#grid_wrapper');
  const grid = $('#grid');
  const highlight = $('#grid__block__highlight');
  update_highlight(highlight);
  update_grid(grid);
  grid_wrapper.get(0).addEventListener("mousemove", function (event) {
    handle_mousemove(event, highlight);
  });

  // Place block - left click
  grid_wrapper.get(0).addEventListener("mousedown", function (event) {
    if (event.buttons == 1)
      handle_click(event, highlight);
  });

  // Remove block - right click
  grid_wrapper.get(0).addEventListener("contextmenu", function (event) {
    event.preventDefault();
    handle_rightclick(event);
  });

});
