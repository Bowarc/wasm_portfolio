precision mediump float;

uniform float u_r;
uniform float u_g;
uniform float u_b;
uniform float u_a;

void main() {
    gl_FragColor = vec4(u_r, u_g, u_b, u_a);
}