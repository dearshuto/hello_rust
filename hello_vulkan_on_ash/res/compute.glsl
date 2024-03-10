#version 450

layout (local_size_x = 8, local_size_y = 8, local_size_z = 8) in;

layout (binding = 0) buffer Data {
    uint u_Data[];
};

void main(){
    uint index = gl_GlobalInvocationID.x;
    u_Data[index] = index;
}
