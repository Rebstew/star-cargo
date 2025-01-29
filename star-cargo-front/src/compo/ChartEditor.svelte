<script>
	import { onMount } from "svelte";
    import ChartEditor from "./chart-editor";

    let chartEditor = new ChartEditor();
    let { data =  chartEditor.createDefaultChart()} = $props();
    let currentFloor = chartEditor.getCurrentFloor();

    onMount(() => {
        chartEditor.populatePicker();
    });

</script>

<style>
    #chart-editor-grid {
        width: 50vmin;
        height: 50vmin;
        border: 1px solid black;
        background-color: white;
        flex: 1 1 50%;
        max-width: 50%;
    }

    .chart-editor-floor-properties {
        display: flex;
        flex-direction: column;
        flex: 0 0 25%;
    }

    #chart-editor-picker {
        width: 5vw;
        height: 50vh;
        display: flex;
        flex-direction: column;
        flex: 0 0 25%;
    }

    .chart-editor-picker-item {
        margin-left: 1em;
        margin-right: 1em;
        margin-top: 0.5em;
        margin-bottom: 0.5em;

        width: 100%;
        height: 100%;

        border-radius: 5px;
    }

    .chart-editor-flex {
        display: flex;
        justify-content: space-between;
        
    }

</style>

<div id="chart-editor">

    <div class="chart-editor-flex">
        <div id="chart-editor-picker"></div>
        <div id="chart-editor-grid" ondragover={ev => chartEditor.dragOverGrid(ev)} ondrop="{ev => chartEditor.dropOnGrid(ev)}"></div>
        <div id="chart-editor-floor-properties">
            <input type="number" id="chart-editor-floor-properties-width" min="1" placeholder="Width">
            <input type="number" id="chart-editor-floor-properties-height" min="1" placeholder="Height">
            <p id="chart-editor-floor-properties-scu-count">SCU: {currentFloor.scuCount()}</p>
        </div>
    </div>



    <div id="chart-editor-sidebar">
        <button id="chart-editor-save">Save</button>
        <button id="chart-editor-cancel">Cancel</button>
    </div>

    <div id="char-editor-actions">
        <button id="chart-editor-add-floor" onclick="{() => chartEditor.addFloor()}">Add floor</button>
    </div>


</div>