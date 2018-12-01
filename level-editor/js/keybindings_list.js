// Fill shortcuts in table with data from keybindings.json
function fill_shortcuts(json) {
  // Loop through controls json
  Object.keys(json).forEach(function (name) {
    const controls = json[name];
    if (controls.constructor.name == "Array") {
      // Fill appropriate shortcut field
      fill_shortcut(name, controls);

    } else if (controls.constructor.name == "Object") {
      // Loop through sub-group of controls
      Object.keys(controls).forEach(function (sub_name) {
        const sub_controls = controls[sub_name];
        const id_name = name + "__" + sub_name;
        // Fill appropriate shortcut field
        fill_shortcut(id_name, sub_controls);
      });

    }
  });
}

// Actually fill field in table
function fill_shortcut(name, controls) {
  const field = $('#key__' + name);
  if (field.length == 0)  return;
  // Replace some key names
  const text = controls.map(function (key) {
    var ret = key;
    if (key.match(/ /)) {
      ret = ret.replace(/ /, "Space");
    } else if (key.match(/Arrow(Up|Down|Left|Right)/)) {
      const matches = key.match(/Arrow(Up|Down|Left|Right)/);
      matches.forEach(function (match) {
        const dir = "<" + match.match(/Up|Down|Left|Right/)[0].toLowerCase() + ">";
        ret = ret.replace(match, dir);
      });
    } else if (key.match(/0/)) {
      ret = ret.replace(/0/, "0 (zero)");
    }
    return ret;
  }).join(", ");
  field.text(text);
}

$(document).ready(function () {
  $.getJSON("./keybindings.json", function (json) {
    fill_shortcuts(json);
  });
});
