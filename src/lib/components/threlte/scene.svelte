<script lang="ts">
	import { T, useTask } from '@threlte/core';
	import { Grid, interactivity, OrbitControls } from '@threlte/extras';
	import { onMount } from 'svelte';
	import { Spring } from 'svelte/motion';
	import { PerspectiveCamera } from 'three';

	let { camera_intrinsic_matrix = $bindable(), camera_extrinsic_matrix = $bindable() } = $props();

	interactivity();
	const scale = new Spring(1, { stiffness: 0.1, damping: 0.2 });

	let rotation = $state(0);
	useTask((delta) => {
		rotation += delta;
		camera_extrinsic_matrix = camera.matrixWorldInverse;
	});

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
	position={[10, 10, 10]}
	oncreate={(ref) => {
		ref.lookAt(0, 1, 0);
	}}
>
	<OrbitControls />
</T.PerspectiveCamera>

<T.DirectionalLight position={[0, 10, 10]} />

<T.Mesh
	position.y={1}
	rotation.y={rotation}
	scale={scale.current}
	onpointerenter={() => {
		scale.target = 1.5;
	}}
	onpointerleave={() => {
		scale.target = 1;
	}}
	castShadow
>
	<T.BoxGeometry args={[1, 2, 1]} />
	<T.MeshStandardMaterial color="hotpink" />
</T.Mesh>

<Grid sectionColor="#5555ff" sectionThickness={1} cellColor="#eeeeee" gridSize={400} />
