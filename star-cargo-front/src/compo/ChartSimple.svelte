<script>
	import { PUBLIC_SERVER_PROTOCOL, PUBLIC_SERVER_URL } from "$env/static/public";
    import Time from "svelte-time";

	let { data } = $props();

    let creation_date = new Date(data.creation_date);
    let user_locale = navigator.languages[0] || navigator.language || '';
    let creation_date_formatted = creation_date.toLocaleString(user_locale);
</script>

<div class="chart-card">
    <img src="{PUBLIC_SERVER_PROTOCOL}://{PUBLIC_SERVER_URL}/image/{data.id}" alt="Representation of the {data.name}">
    <h4>{data.name}</h4>
    <p>{data.description}</p>
    <a href="chart/{data.id}" class="cta-button">Chart details</a>
    <p>Created <Time title="{creation_date_formatted}" timestamp={data.creation_date} relative/></p>
    <p>Id (for debug): {data.id}</p>
</div>