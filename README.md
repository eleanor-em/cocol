## What is Cocol?
It's a type of Mexican bread. Google it.

OK so I had this crazy idea  a while back -- I was making the argument that additively-homomorphic
ElGamal was actually perfectly fine in practice, and to demonstrate this I decided to see how fast I
could brute force decoding messages.

Naturally I had the idea that you might hope to use GPUs to do this, since it's the same task run
thousands of times. Your options?

* Write a GLSL compute shader to do it (painful)
* Write an OpenCL program to do it (only slightly less painful)
* Wite a CUDA program to do it (less painful, but then you have to deal with C++, and also have to
have an NVIDIA GPU)

The natural solution? Invent my own high-level GPU-first language. It will compile to a GLSL shader
plus a simple driver program to load and run the shader. Below is an example of draft Cocol syntax
to solve the knapsack problem, taking inspiration from Rust and from CUDA (because they had a lot
of really good ideas).  

```
kernel knapsack(device, values: [u32], weights: [u32], capacity: u32, n: u32) -> u32 {
	shared result = [[0..capacity + 1]..n + 1];

	let w = device.global.x;

	for i in 1..n + 1 {
		mut val = result[i - 1][w];
		if weights[i - 1] <= w {
			val = max(val, values[i - 1] + result[i - 1][w - weights[i - 1]]);
		}

		result[i][w] = val;

		sync;
	}

	return result[capacity][n];
}

fn main(device) {
    let values = [795u32, 435, 499, 56, 268, 958, 1495, 425, 1340, 512, 126, 1210, 97, 1281, 922, 915, 557, 709, 1524, 81, 186, 1288, 1075, 1007, 714];
    let weights = [424u32, 876, 248, 1279, 829, 286, 1066, 1371, 384, 315, 762, 182, 289, 914, 419, 997, 1492, 736, 1069, 978, 513, 624, 1146, 482, 224];
    let capacity = 10000;

    device.set_dimensions(capacity, 1, 1);

    print(`{knapsack(values, weights, capacity)}`);
}
```
So far, the project contains the GLSL shader we should be producing, as well as a Rust example of
the aforementioned driver program. It uses Vulkan, so is nicely cross-platform. (I don't know
entirely what the state of Vulkan on MacOS is, but it seems like a solvable problem.)

I've started work on a parser for the language. Watch this space.