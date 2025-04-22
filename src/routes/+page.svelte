<script lang="ts">
	import Scene from "$lib/components/threlte/scene.svelte";
import { Canvas } from "@threlte/core";
	import type { Matrix4 } from "three";

    let camera_intrinsic_matrix: Matrix4;
    let camera_extrinsic_matrix: Matrix4;

</script>

<Canvas>
    <Scene bind:camera_intrinsic_matrix bind:camera_extrinsic_matrix/>
</Canvas>
<p class="text-black text-2xl font-bold font-sans">{JSON.stringify(camera_intrinsic_matrix)}</p>
<p>{JSON.stringify(camera_extrinsic_matrix)}</p>
