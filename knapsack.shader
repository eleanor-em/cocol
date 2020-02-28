#version 450

layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(push_constant) uniform Metadata {
    uint capacity;
    uint num_items;
} metadata;

layout(set = 0, binding = 0) buffer Values {
    uint data[];
} values;
layout(set = 0, binding = 1) buffer Weights {
    uint data[];
} weights;
layout(set = 0, binding = 2) buffer Result {
    uint data[];
} result;

uint get(uint x, uint y, uint size) {
    return x * size + y;
}

void main() {
    uint w = gl_GlobalInvocationID.x;
    uint size = metadata.capacity + 1;

    if (w > 0 && w < size) {
        for (uint i = 1; i <= metadata.num_items; ++i) {
            uint new_value = result.data[get(i - 1, w, size)];
            uint drop_last = result.data[get(i - 1, w - weights.data[i - 1], size)];

            if (weights.data[i - 1] <= w) {
                new_value = max(values.data[i - 1] + drop_last, new_value);
            }

            result.data[get(i, w, size)] = new_value;

            memoryBarrierBuffer();
        }
    }
}