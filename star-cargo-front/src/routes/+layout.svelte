<script>
	let { children } = $props();
</script>

<header>
    <h1>Star Cargo Grid</h1>
    <nav>
        <a href="/">Home</a>
        <a href="/chart/create">Create a chart</a>
        <a href="/charts">Chart list</a>
    </nav>
</header>

{@render children()}

<footer>
    &copy; 2025 StarcarGo Grid. All rights reserved.
</footer>