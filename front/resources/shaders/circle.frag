#ifdef GL_ES
precision mediump float;
#endif


uniform vec2 u_resolution; 
uniform vec2 u_center; 



void main() {
  vec2 center = vec2(0.0);
  vec2 pixelCoord = gl_FragCoord.xy -0.5;
  float circle = step(distance(pixelCoord, u_center), 20.0);

  float red = 0.0;
  if (pixelCoord.x < 10.0) {
    red = 1.0;
  }

  gl_FragColor = vec4(red, circle, 1.0, 1.0 );
}