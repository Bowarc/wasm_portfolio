#ifdef GL_ES
precision mediump float;
#endif

uniform vec4 u_color;

varying vec2 uv;

void main() {
  // tests
  // vec2 center = vec2(0.0);
  
  // Thoses coords should be betwen [0, 0] topleft -> [window_width, window_height], right ? 
  vec2 pixelCoord = gl_FragCoord.xy /* / u_resolution -0.5 */ ;

  float d = distance(uv, vec2(0.5));
  if (d > 0.5){
    discard;
  }
  gl_FragColor = u_color;


  
  // if(d > 0.5){
  //   gl_FragColor = vec4(1, 0, 0, d);
  // }else{
  //   gl_FragColor = vec4(1.0, 1.0 , 1.0, 1.0);
  // }

  // gl_FragColor = vec4(1, 1.0 , 1.0, 0.5 - d);

  // if (abs(uv2.x) > 0.1){
  //   discard;
  // }

}
