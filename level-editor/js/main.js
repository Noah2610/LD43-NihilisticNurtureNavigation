// Copy object recursively
//  https://stackoverflow.com/a/24648941
function deep_copy(o) {
  const gdcc = "__getDeepCircularCopy__";
  if (o !== Object(o)) {
    return o; // primitive value
  }

  var set = gdcc in o,
    cache = o[gdcc],
    result;
  if (set && typeof cache == "function") {
    return cache();
  }
  // else
  o[gdcc] = function() { return result;  }; // overwrite
  if (o instanceof Array) {
    result = [];
    for (var i=0; i<o.length; i++) {
      result[i] = deep_copy(o[i]);
    }
  } else {
    result = {};
    for (var prop in o)
      if (prop != gdcc)
        result[prop] = deep_copy(o[prop]);
    else if (set)
      result[prop] = deep_copy(cache);
  }
  if (set) {
    o[gdcc] = cache; // reset
  } else {
    delete o[gdcc]; // unset again
  }
  return result;
}

function copy_to_clipboard(text) {
  var textArea = document.createElement("textarea");
  // Place in top-left corner of screen regardless of scroll position.
  textArea.style.position = 'fixed';
  textArea.style.top = 0;
  textArea.style.left = 0;
  // Ensure it has a small width and height. Setting to 1px / 1em
  // doesn't work as this gives a negative w/h on some browsers.
  textArea.style.width = '2em';
  textArea.style.height = '2em';
  // We don't need padding, reducing the size if it does flash render.
  textArea.style.padding = 0;
  // Clean up any borders.
  textArea.style.border = 'none';
  textArea.style.outline = 'none';
  textArea.style.boxShadow = 'none';
  // Avoid flash of white box if rendered for any reason.
  textArea.style.background = 'transparent';

  textArea.value = text;
  document.body.appendChild(textArea);
  textArea.select();
  document.execCommand('copy');
  document.body.removeChild(textArea);
}

function current_block() {
  const select = $('#panel__block_selector');
  return select.find('option:selected').val();
}

function save_level() {
  $('#level_data').remove();

  const blocks_el = $('#blocks').children('.block');
  var blocks = [];

  blocks_el.each(function (index) {
    const b = $(this);
    if (b.data('instance') && !(parseInt(b.css('width')) == 0 || parseInt(b.css('height')) == 0 )) {
      var bdata = {
        type: b.data('instance'),
        position: {
          x: parseInt(b.css('left')),
          y: parseInt(b.css('top'))
        },
        size: {
          w: parseInt(b.css('width')),
          h: parseInt(b.css('height'))
        }
      };
      var filenames = parse_filenames(b.data('files'));
      if (filenames)
        bdata.images = filenames;
      if (b.data('additional') && !b.data('additional').match(/^\s*$/))
        bdata.additional = JSON.parse(b.data('additional'))
      blocks.push(bdata);
    }
  });

  const data = {
    size: {
      w: settings.room_size.w,
      h: settings.room_size.h
    },
    instances: blocks
  };
  const data_string = JSON.stringify(data);
  // Copy level data to clipboard
  copy_to_clipboard(data_string);
  // Create textarea with level data
  const data_el = $(document.createElement('textarea'));
  data_el.attr("id", "level_data");
  data_el.attr("rows", "20");
  data_el.text(data_string);
  $('#panel__settings').append(data_el);

  // Get level name
  var name = $('#level_name').val();
  if (name.search(/\.json$/) == -1)
    name += ".json";

  // Prompt user for download
  download_json(data, name);
}

function parse_filenames(text) {
  if (!text)
    return false;
  var filenames = text.replace(/(^\s*)|(\s*$)/g, '').split(/\s*,\s*/);
  if (filenames.length > 0)
    return filenames;
  return false;
}

function parse_numbers(text) {
  if (!text)
    return false;
  var numbers = text.replace(/(^\s*)|(\s*$)/g, '').split(/\s*,\s*/).map(function (string) {
    return Number(string);
  });
  if (numbers.length > 0)
    return numbers;
  return false;
}

// https://stackoverflow.com/a/30800715
function download_json(exportObj, exportName){
  var dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(exportObj));
  var dlAnchorElem = document.getElementById('download_level');
  dlAnchorElem.setAttribute("href",     dataStr   );
  dlAnchorElem.setAttribute("download", exportName);
  dlAnchorElem.click();
}

$(document).ready(function () {
  const save_btn = $('#save_btn');

  save_btn.get(0).addEventListener("click", save_level);
});
