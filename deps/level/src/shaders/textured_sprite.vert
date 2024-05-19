
layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 in_tex_coord;

uniform bool flipped;
uniform bool flipped_y;
uniform vec2 size;
uniform vec2 position;
uniform float scale;
uniform float rotation;
uniform float camera_rotation;
uniform vec2 camera_position;

uniform vec2 resolution;

out vec2 tex_coord;

mat4 rotation_z_matrix(in float angle) {
    float cos_z = cos(angle);
    float sin_z = sin(angle);
    return mat4 (
    cos_z, sin_z, 0, 0,
    -sin_z, cos_z, 0, 0,
    0,     0, 1, 0,
    0,     0, 0, 1
    );
}


void main() {

    gl_Position = vec4(vertex_position.xy, 1.0, 1.0);

    if (flipped) {
        gl_Position.x *= -1.0;
    }

    if (flipped_y) {
        gl_Position.y *= -1.0;
    }

    gl_Position.x *= size.x;
    gl_Position.y *= size.y;

    gl_Position *= rotation_z_matrix(-rotation);

    gl_Position.xy += position - camera_position;

    gl_Position *= rotation_z_matrix(camera_rotation);

    gl_Position.x *= resolution.y / resolution.x;

    gl_Position.xy *= scale;
    float scale = resolution.y / 10.0;
    gl_Position.xy /= scale;

    tex_coord = in_tex_coord;
}