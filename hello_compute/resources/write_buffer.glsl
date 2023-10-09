#version 450

layout (local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout (binding = 0, std430) buffer Buffer
{
    uint u_Buffer[];
};

// 等差数列を計算する演算シェーダ
void main()
{
	uint result = gl_GlobalInvocationID.x * 3;
	u_Buffer[gl_GlobalInvocationID.x] = result;
}
