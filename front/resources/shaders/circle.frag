#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution; 
uniform vec2 u_center; 

void main() {
  // tests
  // vec2 center = vec2(0.0);
  
  vec2 pixelCoord = gl_FragCoord.xy;
  float circle = step(distance(pixelCoord, u_center), 20.0);

  gl_FragColor = vec4(1.0, 1.0 , 1.0, circle );
}