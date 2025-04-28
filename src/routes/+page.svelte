<script lang="ts">
	import Scene from '$lib/components/threlte/scene.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Canvas } from '@threlte/core';
	import type { Matrix4 } from 'three';
    import { open } from '@tauri-apps/plugin-dialog';

	let camera_intrinsic_matrix: Matrix4;
	let camera_extrinsic_matrix: Matrix4;

    const loadDirectory = async () => {
        const selected = await open({
            multiple: false,
            directory: true,
            filters: [
                {
                    name: 'JSON',
                    extensions: ['json']
                }
            ]
        });

        console.log(selected);
    };

</script>

<Canvas>
	<Scene bind:camera_intrinsic_matrix bind:camera_extrinsic_matrix />
</Canvas>
<div class="flex flex-row">
    <Button variant="outline" onclick={() => loadDirectory()}>
        Load Files
    </Button>
</div>
<p class="font-sans text-2xl font-bold text-black">{JSON.stringify(camera_intrinsic_matrix)}</p>
<p>{JSON.stringify(camera_extrinsic_matrix)}</p>
