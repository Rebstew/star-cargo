<script>

	let { upvotes, downvotes, classes=""} = $props();

    let total_votes = upvotes + downvotes;

</script>

<progress class={classes} value={upvotes} max={total_votes}></progress>