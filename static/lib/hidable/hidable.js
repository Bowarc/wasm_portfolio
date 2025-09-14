window.addEventListener("scroll", function () {
  update_visibles();
});

// Wasm handles that part
// window.onload = function () {
//   update_visibles();
// };

function is_visible(elm, offset) {
  var rect = elm.getBoundingClientRect();
  var viewHeight = Math.max(document.documentElement.clientHeight, window.innerHeight);
  return !(rect.bottom < 0 - offset || rect.top - viewHeight >= 0 + offset);
}

function update_visibles() {
  let offset = Math.abs(checkScrollSpeed() * 10);
  var elements = document.getElementsByClassName("hidable_element");
  for (var i = 0; i < elements.length; i++) {
    el = elements.item(i)
    if (is_visible(el, offset)) {
      el.classList.add("visible_element");
        el.classList.remove("hidden_element");
    } else {
      el.classList.add("hidden_element");
      el.classList.remove("visible_element");
    }
  }
}


// Source: https://stackoverflow.com/a/22599173
var checkScrollSpeed = (function(settings){
    settings = settings || {};
  
    var lastPos, newPos, timer, delta, 
        delay = settings.delay || 150; // in "ms" (higher means lower fidelity )
  
    function clear() {
      lastPos = null;
      delta = 0;
    }
  
    clear();
    
    return function(){
      newPos = window.scrollY;
      if ( lastPos != null ){ // && newPos < maxScroll 
        delta = newPos -  lastPos;
      }
      lastPos = newPos;
      clearTimeout(timer);
      timer = setTimeout(clear, delay);
      return delta;
    };
})();
