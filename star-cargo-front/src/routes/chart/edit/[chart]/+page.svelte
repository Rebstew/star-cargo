<script>
	import ChartEditor from "../../../../compo/ChartEditor.svelte";

    let { chart } = $props();

</script>

<svelte:head>
    <title>Edit a chart -- Star Cargo Grid</title>
</svelte:head>

<h2>Edit a chart</h2>

<ChartEditor data={chart}/>