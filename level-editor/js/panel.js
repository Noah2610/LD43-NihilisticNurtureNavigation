function init_panel(panel) {
  [
    panel.find('#panel__box_size__w'),
    panel.find('#panel__box_size__h'),
    panel.find('#panel__block_size__w'),
    panel.find('#panel__block_size__h'),
    panel.find('#panel__block_offset__x'),
    panel.find('#panel__block_offset__y'),
    panel.find('#panel__room_size__w'),
    panel.find('#panel__room_size__h'),
    panel.find('#panel__additional_json')
  ].forEach(function (el) {
    el.get(0).addEventListener("change", function (event) {
      const input = $(event.target);
      const val = el.val();
      switch (input.attr("id")) {
        case 'panel__box_size__w':
          settings.box_size.w = parseInt(val);
          break;
        case 'panel__box_size__h':
          settings.box_size.h = parseInt(val);
          break;
        case 'panel__block_size__w':
          settings.block_size.w = parseInt(val);
          break;
        case 'panel__block_size__h':
          settings.block_size.h = parseInt(val);
          break;
        case 'panel__block_offset__x':
          settings.block_offset.x = parseInt(val);
          break;
        case 'panel__block_offset__y':
          settings.block_offset.y = parseInt(val);
          break;
        case 'panel__room_size__w':
          settings.room_size.w = parseInt(val);
          break;
        case 'panel__room_size__h':
          settings.room_size.h = parseInt(val);
          break;
        case 'panel__additional_json':
          console.log(val);
          settings.additional_json = parse_json(val);
          break;
      }

      update_panel(panel);
      update_grid($('#grid'));
    });
  });

  update_panel(panel);
}

function parse_json(text) {
  return JSON.parse(text);
}

function update_panel(panel = $('#panel')) {
  const box_size_el = {
    w: panel.find('#panel__box_size__w'),
    h: panel.find('#panel__box_size__h')
  };

  const block_size_el = {
    w: panel.find('#panel__block_size__w'),
    h: panel.find('#panel__block_size__h')
  };

  const block_offset_el = {
    x: panel.find('#panel__block_offset__x'),
    y: panel.find('#panel__block_offset__y')
  };

  const room_size_el = {
    w: panel.find('#panel__room_size__w'),
    h: panel.find('#panel__room_size__h')
  };

  box_size_el.w.val(settings.box_size.w);
  box_size_el.h.val(settings.box_size.h);
  block_size_el.w.val(settings.block_size.w);
  block_size_el.h.val(settings.block_size.h);
  block_offset_el.x.val(settings.block_offset.x);
  block_offset_el.y.val(settings.block_offset.y);
  room_size_el.w.val(settings.room_size.w);
  room_size_el.h.val(settings.room_size.h);

  update_highlight();
}

function populate_block_selector() {
  const block_selector = $('#panel__block_selector__select');
  $.getJSON('./instance_list.json', function (json) {
    json.forEach(function (instance) {
      const name = instance.name;
      const color = instance.color;
      settings.colors[name] = color;
      const option = $(document.createElement("option"));
      option.addClass("panel__block_selector__select__option");
      option.val(name);
      option.text(name);
      option.css("color", color);
      block_selector.append(option);

      block_selector_update_color(block_selector.get(0), json);

      // EventListener for color changing when selecting option
      block_selector.on("change", function (event) {
        block_selector_update_color(event.target, json);
      });
    });
  });
}

function block_selector_update_color(target, json) {
  const select = $(target);
  json.forEach(function (instance) {
    if (instance.name == select.val()) {
      select.css("color", instance.color);
      $('#grid__block__highlight').css("background-color", instance.color);
      return;
    }
  });
}

function load_level(data) {
  const json = JSON.parse(data);
  const block_wrapper = $('#blocks')
  block_wrapper.empty();

  if (json.size) {
    if (json.size.w) {
      settings.room_size.w = json.size.w;
      $('#panel__room_size__w').val(json.size.w);
    }
    if (json.size.h) {
      settings.room_size.h = json.size.h;
      $('#panel__room_size__h').val(json.size.h);
    }
  }

  // Generate DOMs
  json.instances.forEach(function (instance) {
    const x = instance.position.x + "px";
    const y = instance.position.y + "px";
    const w = instance.size.w + "px";
    const h = instance.size.h + "px";
    const type = instance.type;
    const color = settings.colors[instance.type];

    const block = $(document.createElement("span"));
    block.addClass("grid__box block");
    block.css("left", x);
    block.css("top", y);
    block.css("width", w);
    block.css("height", h);
    block.css("background-color", color);
    block.data("instance", type);

    block_wrapper.append(block);
  });

  update_grid();
}

function load_level_file(event) {
  const file = event.target.files[0];
  const freader = new FileReader();
  freader.onload = function (event) {
    const data = event.target.result;
    load_level(data);
  }
  freader.readAsText(file);
}

function toggle_panel(event) {
  const btn = $('#panel__toggle_panel');
  const panel_wrapper = $('#panel');
  const panel = $('#panel__settings');
  if (panel.css("display") == "block") {
    // Hide panel
    panel.css("display", "none");
    panel_wrapper.css("height", "auto");
    panel_wrapper.css("overflow-y", "auto");
    btn.val("<");
  } else if (panel.css("display") == "none") {
    // Show panel
    panel.css("display", "block");
    panel_wrapper.css("height", "95%");
    panel_wrapper.css("overflow-y", "scroll");
    btn.val(">");
  }
}

$(document).ready(function () {

  const panel = $('#panel')
  init_panel(panel);

  populate_block_selector();

  // Level uploading
  $('#load_level').get(0).addEventListener("change", load_level_file);

  // Toggle panel
  $('#panel__toggle_panel').get(0).addEventListener("click", toggle_panel);

});
