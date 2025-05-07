<script lang="ts">
	import Scene from '$lib/components/threlte/scene.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Canvas, T } from '@threlte/core';
	import type { Matrix4 } from 'three';
    import { open } from '@tauri-apps/plugin-dialog';
    import { readFile } from '@tauri-apps/plugin-fs';
    import { readDir, readTextFile } from '@tauri-apps/plugin-fs';
	import { invoke } from '@tauri-apps/api/core';
	import type { Mesh } from '$lib/types/mesh';
    import { OBJLoader, PLYLoader, STLLoader } from 'three/examples/jsm/Addons.js';


	let camera_intrinsic_matrix: Matrix4;
	let camera_extrinsic_matrix: Matrix4;

    let file_paths: (string | undefined)[] = $state([]);
    let meshData: Promise<Mesh> | null = $state(null);

    const loadDirectory = async () => {
        console.log("Running loadDirectory ");
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

        if (selected && typeof selected === 'string') {
            try {
                const entries = await readDir(selected, { });
                file_paths = await Promise.all(
                    entries.map(async (entry) => {
                        if (entry && entry.name) {
                            const filePath = `${selected}/${entry.name}`;
                            return filePath;
                        }
                    })
                );

                file_paths = file_paths.filter((file) => file !== undefined);
            } catch (error) {
                console.error("Error reading directory: ", error);
            }
        }
        
        if (file_paths) {
            // meshData = invoke('load_mesh_file', { path: file_paths[0] });
            let file = await readFile(file_paths[0])
            let blob = new Blob([file])
            let url = await URL.createObjectURL(blob)
            let loader = new OBJLoader()
            loader.load(url, (res) => {
                console.log(res)
                meshData = res
            })
        }
    };

</script>
<div class="flex flex-row">
    <Button variant="outline" onclick={() => loadDirectory()}>
        Load Files
    </Button>
</div>
<Canvas>
	<Scene bind:camera_intrinsic_matrix bind:camera_extrinsic_matrix bind:meshData />
</Canvas>
<!-- <p class="font-sans text-2xl font-bold text-black">{JSON.stringify(camera_intrinsic_matrix)}</p>
<p>{JSON.stringify(camera_extrinsic_matrix)}</p> -->
