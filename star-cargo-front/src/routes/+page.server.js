import { PUBLIC_SERVER_PROTOCOL, PUBLIC_SERVER_URL } from '$env/static/public';

export async function load() {

    let popularCharts = [];

    console.log('Fetching popular entries from public server...');

    await fetch(PUBLIC_SERVER_PROTOCOL + '://' + PUBLIC_SERVER_URL + '/popularEntries')
        .then(response => console.log(response.status) || response)
        .then(response => response.json())
        .then(response => {
            console.log('Response json is: ', response);
            for (let chart of response) {
                popularCharts.push(renameIdFields([chart])[0]);
            }
        });

        console.log('Popular charts are, popularCharts: ', popularCharts);

        return {
            popularCharts: popularCharts
        };
}

function renameIdFields(objects) {
    console.log('Renaming id fields... objects: ', objects);
    return objects.map(({ _id, ...rest }) => ({
        id: _id.$oid, // Extract the value from the `foo` key in the `_id` object
        ...rest      // Spread the remaining fields
    }));
}