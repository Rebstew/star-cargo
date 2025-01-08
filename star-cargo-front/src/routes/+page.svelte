<script>
	let { data } = $props();
</script>

<div class="main-content">
    <h2>Star Cargo</h2>

    <p>Besoin d'optimiser votre cargo bla bla bla. StarcarGo grid est fait pour vous ! 
        Créez votre Schéma sur le vaisseau de vos rêves bla bla bla.</p>
    <a href="chartEdit" class="cta-button">Create your first chart</a>
</div>

<section class="chart-section">
    <h3>Most popular charts</h3>
    <div class="chart-grid">
        {#each data.popularCharts as popularChart}
        <div class="chart-card">
            <img src="assets/images/ship.jpg" alt="Ship image">
            <h4>{popularChart.name}</h4>
            <p>{popularChart.description}</p>
            <a href="chart/{popularChart.id}" class="cta-button">Voir le schéma</a>
        </div>
        {/each}
    </div>
</section>