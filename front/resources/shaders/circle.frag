#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution; 
uniform vec2 u_center; 
uniform float u_radius; 

void main() {
  // tests
  // vec2 center = vec2(0.0);
  
  // Thoses coords should be betwen [0, 0] topleft -> [window_width, window_height], right ? 
  vec2 pixelCoord = gl_FragCoord.xy /* / u_resolution -0.5 */ ;

  float d = distance(pixelCoord, u_center);
  
  // This is not woring as expected
  // if(d > u_radius){
  //   discard;
  // }

  gl_FragColor = vec4(1.0, 1.0 , 1.0, 1.0);
}