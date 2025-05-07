<script lang="ts">
	import { T, useTask } from '@threlte/core';
	import { Environment, Grid, interactivity, OrbitControls } from '@threlte/extras';
	import { onMount } from 'svelte';
	import { Spring } from 'svelte/motion';
	import { BufferGeometry, PerspectiveCamera, Vector3 } from 'three';

	let {
		camera_intrinsic_matrix = $bindable(),
		camera_extrinsic_matrix = $bindable(),
		meshData = $bindable(),
        mesh_path = $bindable()
	} = $props();

	// interactivity();
	// const scale = new Spring(1, { stiffness: 0.1, damping: 0.2 });

	// let rotation = $state(0);
	// useTask((delta) => {
	// 	rotation += delta;
	// 	camera_extrinsic_matrix = camera.matrixWorldInverse;
	// });

	let camera: PerspectiveCamera = $state(new PerspectiveCamera());

	onMount(() => {
		if (camera) {
			camera_intrinsic_matrix = camera.projectionMatrix;
			camera_extrinsic_matrix = camera.matrixWorldInverse;
		}
	});

    

</script>

<T.PerspectiveCamera
	bind:ref={camera}
	makeDefault
	position={[1, 1, 1]}
	oncreate={(ref) => {
		ref.lookAt(0, 0, 0);
	}}
>
	<OrbitControls />
</T.PerspectiveCamera>

<T.DirectionalLight position={[0, 10, 0]} />

<Environment isBackground={false} url="/hdri.exr" />

<T is={meshData}/>

<!-- {#if meshData}
	{#await meshData then meshData}
		<T.Mesh castShadow={true}>
			<T.BufferGeometry>
				<T.BufferAttribute
					args={[Float32Array.from(meshData.vertices), meshData.vertices_size]}
					attach={({ parent, ref }) => {
						parent.setAttribute('position', ref);
						return () => {
							parent.deleteAttribute('position');
						};
					}}
				/>
				<T.BufferAttribute attach="index" args={[Uint16Array.from(meshData.faces), 1]} />
				{#if meshData.colours}
					<T.BufferAttribute
						args={[Float32Array.from(meshData.colours), meshData.colour_size]}
						attach={({ parent, ref }) => {
							parent.setAttribute('color', ref);
							return () => {
								parent.deleteAttribute('color');
							};
						}}
					/>
				{/if}
				{#if meshData.normals}
					<T.BufferAttribute
						args={[Float32Array.from(meshData.normals), meshData.normal_size]}
						attach={({ parent, ref }) => {
							parent.setAttribute('normal', ref);
							return () => {
								parent.deleteAttribute('normal');
							};
						}}
					/>
				{/if}
			</T.BufferGeometry>
            <T.MeshPhongMaterial />
		</T.Mesh>
	{/await}
	
{/if} -->

<!-- rotation.y={rotation}
    scale={scale.current}
    onpointerenter={() => {
        scale.target = 1.5;
    }}
    onpointerleave={() => {
        scale.target = 1;
    }} -->

<Grid sectionColor="#69cdff" sectionThickness={1} cellColor="#eeeeee" infiniteGrid={true} receiveShadow={true}/>
