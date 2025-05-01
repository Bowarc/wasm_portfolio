window.addEventListener("scroll", function () {
  update_visibles();
});

// Wasm handles that part
// window.onload = function () {
//   update_visibles();
// };

function is_visible(elm) {
  var rect = elm.getBoundingClientRect();
  var viewHeight = Math.max(document.documentElement.clientHeight, window.innerHeight);
  return !(rect.bottom < 0 || rect.top - viewHeight >= 0);
}

function update_visibles() {
  var elements = document.getElementsByClassName("hidable_element");
  for (var i = 0; i < elements.length; i++) {
    el = elements.item(i)
    if (is_visible(el)) {
      el.classList.add("visible_element");
        el.classList.remove("hidden_element");
    } else {
      el.classList.add("hidden_element");
      el.classList.remove("visible_element");
    }
  }
}
